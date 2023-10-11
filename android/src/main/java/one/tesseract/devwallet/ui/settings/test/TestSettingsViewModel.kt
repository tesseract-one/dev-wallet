package one.tesseract.devwallet.ui.settings.test

import android.util.Log
import androidx.lifecycle.*
import one.tesseract.devwallet.entity.TestSettings
import one.tesseract.devwallet.rust.TestSettingsProvider

class TestSettingsViewModel : ViewModel() {
    private val _cache = MutableLiveData<TestSettings>().apply {
        value = TestSettings("_signed_by_tesseract", "error")
    }
    val cache: MutableLiveData<TestSettings> = _cache

    private val _signature = MutableLiveData<String>().apply {
        value = cache.value?.signature
    }
    val signature: MutableLiveData<String> = _signature

    private val _invalidator = MutableLiveData<String>().apply {
        value = cache.value?.invalidator
    }
    val invalidator: MutableLiveData<String> = _invalidator

    val dirty: LiveData<Boolean> = signature.switchMap { signature ->
        invalidator.switchMap { invalidator ->
            cache.map { cache ->
                (!signature.equals(cache.signature)) || (!invalidator.equals(cache.invalidator))
            }
        }
    }

    //var saved = cache.value!!

    lateinit var provider: TestSettingsProvider

    fun save() {
        val settings = TestSettings(signature.value!!, invalidator.value!!)

        //TODO: write to file (in rust)

        provider.save(settings)

//        saved = settings
//        Log.d("SAVE", "Sig " + settings.signature)
//        Log.d("SAVE", "Inv " + settings.invalidator)

        cache.value = settings
    }

    fun load() {
        //TODO: load from file (in rust)
        val loaded = provider.load()//saved

        cache.value = loaded
        signature.value = loaded.signature
        invalidator.value = loaded.invalidator
    }
}