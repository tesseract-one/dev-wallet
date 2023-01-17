package one.tesseract.devwallet

import one.tesseract.devwallet.rust.Core

class Application: android.app.Application() {
    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }

    lateinit var core: Core

    override fun onCreate() {
        super.onCreate()

        core = createCore(applicationInfo.dataDir)
    }

    private external fun createCore(dataDir: String): Core
}