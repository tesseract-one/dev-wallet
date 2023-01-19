package one.tesseract.devwallet

import one.tesseract.devwallet.rust.Core

class Application: android.app.Application() {
    lateinit var core: Core

    override fun onCreate() {
        super.onCreate()

        core = Core.create(UI(this), applicationInfo.dataDir)
    }
}