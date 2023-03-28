package one.tesseract.devwallet

import android.os.Parcelable
import java.util.concurrent.CompletionStage

import one.tesseract.ipc.activity.ActivityMonitor
import one.tesseract.ipc.activity.free.Launcher

import one.tesseract.devwallet.ui.sign.SignActivity

@Suppress("unused") //The class in used from Rust
class UI(application: Application) {
    private val launcher: Launcher = Launcher(ActivityMonitor(application))

    fun <T: Parcelable>requestUserConfirmation(request: T): CompletionStage<Boolean> {
        return SignActivity.requestUserConfirmation(launcher, request)
    }
}