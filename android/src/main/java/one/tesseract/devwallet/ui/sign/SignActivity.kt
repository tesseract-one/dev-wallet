package one.tesseract.devwallet.ui.sign

import android.os.Bundle

import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.commit

import one.tesseract.devwallet.R
import one.tesseract.devwallet.ui.settings.home.HomeFragment

class SignActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        if(savedInstanceState == null) {
            supportFragmentManager.commit {
                setReorderingAllowed(true) //must be here. otherwise compat mode
                replace(R.id.transactionFragmentContainerView, HomeFragment())
            }
        }

        setContentView(R.layout.activity_sign)
    }
}