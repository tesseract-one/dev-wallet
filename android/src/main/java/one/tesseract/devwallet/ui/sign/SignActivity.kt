package one.tesseract.devwallet.ui.sign

import java.util.concurrent.CompletionStage

import android.os.Bundle
import android.os.Parcelable
import android.widget.Button

import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.commit

import one.tesseract.activity.detached.Launcher
import one.tesseract.activity.detached.finishDetachedActivity

import one.tesseract.devwallet.R
import one.tesseract.devwallet.entity.request.SubstrateAccount
import one.tesseract.devwallet.entity.request.SubstrateSign
import one.tesseract.devwallet.entity.request.TestError
import one.tesseract.devwallet.entity.request.TestSign
import one.tesseract.devwallet.ui.sign.fragments.substrate.account.SubstrateAccountFragment
import one.tesseract.devwallet.ui.sign.fragments.substrate.sign.SubstrateSignFragment
import one.tesseract.devwallet.ui.sign.fragments.test.error.TestErrorFragment
import one.tesseract.devwallet.ui.sign.fragments.test.sign.TestSignFragment

class SignActivity : AppCompatActivity() {
    companion object {
        private const val REQUEST = "request"

        fun <T: Parcelable>requestUserConfirmation(launcher: Launcher, request: T): CompletionStage<Boolean> {
            val bundle = Bundle()

            bundle.putParcelable(REQUEST, request)

            return launcher.startDetachedActivityForResultFuture<Boolean>(SignActivity::class.java, bundle).thenApply {
                it.second
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        @Suppress("DEPRECATION")
        val request: Any = extras.getParcelable(REQUEST) ?: throw RuntimeException("No Request")

        val fragment = when (request) {
            is TestSign -> {
                TestSignFragment(request)
            }
            is TestError -> {
                TestErrorFragment(request)
            }
            is SubstrateAccount -> {
                SubstrateAccountFragment(request)
            }
            is SubstrateSign -> {
                SubstrateSignFragment(request)
            }
            else -> {
                throw RuntimeException("Please, don't send garbage here")
            }
        }

        if(savedInstanceState == null) {
            supportFragmentManager.commit {
                setReorderingAllowed(true) //must be here. otherwise compat mode
                replace(R.id.transactionFragmentContainerView, fragment)
            }
        }

        setContentView(R.layout.activity_sign)

        val buttonSign = findViewById<Button>(R.id.buttonSign)
        val buttonCancel = findViewById<Button>(R.id.buttonCancel)

        buttonSign.setOnClickListener {
            this.finishDetachedActivity(RESULT_OK, true)
        }

        buttonCancel.setOnClickListener {
            this.finishDetachedActivity(RESULT_CANCELED, false)
        }
    }
}