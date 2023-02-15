package one.tesseract.devwallet.ui.settings.key

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import one.tesseract.devwallet.Application
import one.tesseract.devwallet.databinding.FragmentKeySettingsBinding

class KeySettingsFragment : Fragment() {
    private var _binding: FragmentKeySettingsBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val viewModel = ViewModelProvider(this).get(KeySettingsViewModel::class.java)

        //late bind
        val application = this.activity?.application as Application
        viewModel.provider = application.core.keySettingsProvider()

        viewModel.load()

        _binding = FragmentKeySettingsBinding.inflate(inflater, container, false)
        binding.model = viewModel
        binding.lifecycleOwner = this

        return binding.root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}