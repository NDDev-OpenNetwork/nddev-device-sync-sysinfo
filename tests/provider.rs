#![cfg(target_os = "linux")]
use nddev_device_sync_sysinfo::{CancellationToken, NativeIo, ReadSystemInfo, SystemInfo};
use std::{
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
struct OwnedDaemon(Child);
impl Drop for OwnedDaemon {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::test]
#[ignore = "requires NDS_TEST_SYSINFO_DAEMON pointing to the real native provider"]
async fn reads_real_native_sample_on_an_isolated_socket() {
    let executable =
        std::env::var_os("NDS_TEST_SYSINFO_DAEMON").expect("native sysinfo daemon required");
    assert!(std::path::Path::new(&executable).is_absolute());
    let fixture = tempfile::tempdir().unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(fixture.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let mut daemon = OwnedDaemon(
        Command::new(executable)
            .env("XDG_RUNTIME_DIR", fixture.path())
            .env_remove("LISTEN_PID")
            .env_remove("LISTEN_FDS")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let provider = SystemInfo::new(
        NativeIo::new(2).unwrap().local_handle(),
        fixture.path().join("rldyour-sysinfo.sock"),
    )
    .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    let sample = loop {
        if let Ok(sample) = provider.sample(CancellationToken::new()).await {
            break sample;
        }
        assert!(
            Instant::now() < deadline && daemon.0.try_wait().unwrap().is_none(),
            "native provider failed to become ready"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(sample.v, 1);
    // Real measurements are not embedded as fixtures or printed into CI logs.
}
