package one.tesseract.devwallet

class Application: android.app.Application() {
    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }

    private external fun hello(dataDir: String)

    override fun onCreate() {
        super.onCreate()

        hello("here we go!")
    }
}