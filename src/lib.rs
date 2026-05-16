// Link your existing logic modules into the library environment
pub mod telemetry;
pub mod interceptor;
pub mod entropy;
pub mod watchdog;
pub mod rollback;
pub mod atomic_rollback;
pub mod self_protection;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

/// 📱 A secure Android FFI JNI binding wrapper
/// This lets an Android app interface directly with your core telemetry loop at native speeds.
#[no_mangle]
pub extern "system" fn Java_com_omniguard_ai_SentinelBridge_runNativeCheck(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let payload = telemetry::collect_telemetry();
    let response = format!("Sentinel Native Active. Status: {}", payload.status);
    
    env.new_string(response)
        .expect("Failed to create Java String")
        .into_raw()
}