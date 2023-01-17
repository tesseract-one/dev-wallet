package one.tesseract.devwallet

import one.tesseract.devwallet.rust.TestSettingsProvider

class Application: android.app.Application() {
    lateinit var testSettingsProvider: TestSettingsProvider

    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }

    private external fun hello(dataDir: String)

    private external fun createTestSettingsProvider(dataDir: String): TestSettingsProvider

    override fun onCreate() {
        super.onCreate()

        hello("here we go!")

        testSettingsProvider = createTestSettingsProvider(this.applicationInfo.dataDir)


    }
}