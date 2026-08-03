using System;
using System.Runtime.InteropServices;

public static class TransitionNative {
    [DllImport("transition_core.dll", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    public static extern int RenderTransition(string name, float progress, IntPtr outPixels);
}

public class WmmrTransitionPlugin {
    // Minimal OFX stub – real OFX entry points omitted for brevity.
    public static int OfxPluginMain(string action, IntPtr handle) {
        // Placeholder: return success.
        return 0;
    }
}
