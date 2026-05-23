package com.omniguard.sentinel;

import android.app.Activity;
import android.os.Bundle;

public class MainActivity extends Activity {
    // This perfectly matches the libsentinel_core.so file compiled earlier
    static {
        System.loadLibrary("sentinel_core");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // The UI is headless for now; the Rust engine runs silently in the background
    }
}
