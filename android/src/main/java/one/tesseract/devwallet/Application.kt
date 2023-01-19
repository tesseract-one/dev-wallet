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

        val ui = UI(this)

        core = createCore(ui, applicationInfo.dataDir)
    }

    private external fun createCore(ui: UI, dataDir: String): Core
}