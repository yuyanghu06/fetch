# Rendering the actual Figma shadows

The user rejected diagonal gradients and synthetic hard bevels. The design uses uniform fills plus complete outer and inner shadow stacks. Source values are in the fresh official Figma contexts; no screenshots are used as app backgrounds.

The existing [resvg0.45.1 filters](https://github.com/linebender/resvg/blob/v0.45.1/crates/resvg/src/filter/mod.rs) implement blur, offset, flood, alpha inversion/compositing and merging. The library is Apache-2.0 OR MIT licensed and already installed through Iced SVG; a direct dependency pins the same version. No dependency upgrade is selected. CSS blur radius maps to half that value as Gaussian standard deviation, per [the CSS shadow definition](https://www.w3.org/TR/css-backgrounds-3/#shadow-blur). Filter colors use sRGB and enough transparent bounds to keep soft shadows uncut.

First real equal-size captures revealed a library problem that numerical style matching alone did not catch. [Iced's SVG path](https://github.com/iced-rs/iced/blob/0.14.0/wgpu/src/image/vector.rs) uploads resvg's premultiplied color channels unchanged, while its [image pipeline](https://github.com/iced-rs/iced/blob/0.14.0/wgpu/src/image/mod.rs) uses source-alpha blending. This multiplies transparency twice, creating dark halos and dark antialiased edges. Independent Researcher and Tester confirmed this from the installed version and actual captures. Stock Button shadows/default styles were ruled out: default Color alpha is zero.

The scoped correction renders only generated decoration SVGs with the same resvg library, converts its color channels back to ordinary RGBA once, and draws them through Iced's raw-image path. Native controls still own all editing, input, clipboard and overlay behavior. Per-widget cached decoration avoids repeating filters on unchanged surfaces; no third-party/vendor source is patched.

Initial diagnostic component/two-page exports took55.77seconds under unoptimized library code, a concrete development-rendering concern. Only resvg/tiny-skia blur/drawing libraries are optimized in the debug profile; Fetch's own code remains an ordinary debug build. Final timings and visual comparison remain required. Initial failed pictures are retained with initial- prefixes. Direct image rendering also needs correct backing scale for Retina; 1x comparisons alone do not establish native high-DPI appearance.

## Additional problems caught by full-page renders

The custom decoration and native children initially shared one Iced drawing layer. Its separate image-filter and primitive batches put some backgrounds above glyphs and native rectangle fills. Decorations now share the SVG image filter batch, explicitly prepare their in-memory image allocation, and draw native foreground children in a later viewport layer. This preserves first-frame server wells, action icons, white numeric arrow plates and filled checkbox boxes. It fixes composition rather than hiding missing controls in screenshots.

Shadow layer order was compared in an A/B button capture. Following the literal CSS back-to-front order slightly reduced measured error and matches the documented design. Inset input padding now includes all shadow support, including the inverted-alpha blur, not only outside shadows. Visible inset output remains clipped to the original rounded face.
