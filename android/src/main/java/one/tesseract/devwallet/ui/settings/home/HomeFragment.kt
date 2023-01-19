package one.tesseract.devwallet.ui.settings.home

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import one.tesseract.devwallet.databinding.FragmentHomeBinding
import one.tesseract.devwallet.entity.request.TestSign
import one.tesseract.devwallet.ui.sign.SignActivity
import one.tesseract.ipc.activity.ActivityMonitor
import one.tesseract.ipc.activity.free.Launcher

class HomeFragment : Fragment() {
    private var _binding: FragmentHomeBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val viewModel = ViewModelProvider(this).get(HomeViewModel::class.java)

        _binding = FragmentHomeBinding.inflate(inflater, container, false)
        binding.model = viewModel
        binding.lifecycleOwner = this

        val launcher = Launcher(ActivityMonitor(requireActivity().application))

        viewModel.open.observe(viewLifecycleOwner) { it?.let {
            val req = TestSign("hardcode_from_home", "sig", "res")
            SignActivity.requestUserConfirmation(launcher, req)
        }}

        return binding.root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}