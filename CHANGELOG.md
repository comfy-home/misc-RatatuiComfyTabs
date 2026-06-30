# Changelog History

Newest archived changelogs first. When multiple archived files represent the same version, only the newest archive is included here.

## Changelog `v0.5.10` <sup><div align="end">🗓️ 2026-06-18</div></sup>

### 💥 💥 💥 This Release's Top Picks ...  💥 💥 💥

#### **1. &nbsp;&nbsp;&nbsp;LIMITER for max number of displayed tabs**
- new optional argument within position declaration `.max(n)`
- you can use for example `HorizontalPosition::Top.max(5))` to cap max displayed tabs to 5
- or `VerticalPosition::Left.max(2))` to limit vertical tabs to 2
- no breaking changes, old config still works as before!
- new feature well documented
  - no GIF this time, but it should be easy to imagine...
  - however, the feature is included in the examle DEMO case (assigned shortcut `3`), feel free to try it first!


<sub>...  🎉 Enjoy!</sub>

<br>

### ✨ New Feature: Limiter for number of displayed tabs

* add HorizontalPositionConfig and VerticalPositionConfig structs with max visibility options for tab navigation <sub><sup><sup>_b6dcd0e_</sup></sup></sub>

* add position max limiter functionality to tab navigation, allowing control over displayed tabs based on mode <sub><sup><sup>_0899cd4_</sup></sup></sub>

* implement max visible tabs and scroll window functionality for improved tab management <sub><sup><sup>_3f0ce8b_</sup></sup></sub>

* add HorizontalPositionConfig and VerticalPositionConfig to public API for enhanced tab positioning options <sub><sup><sup>_3906324_</sup></sup></sub>

* update horizontal and vertical position fields to use config structs for improved type safety and flexibility <sub><sup><sup>_b67b1e2_</sup></sup></sub>

* update navigation position checks to access position field for improved accuracy <sub><sup><sup>_9d348fe_</sup></sup></sub>

* refactor scroll logic to utilize uses_scroll_window for improved clarity and maintainability <sub><sup><sup>_e837d79_</sup></sup></sub>

* add unit tests for horizontal and vertical position max limits to ensure correct tab visibility and scrolling behavior <sub><sup><sup>_810952d_</sup></sup></sub>

---

## 💬 General Improvements & Fixes:

### 🔧 Maintenance

* CG app version bump to v0.5.10 <sub><sup><sup>_ec2cc86_</sup></sup></sub>

### ℹ️ Documentation

* update README with demo setup instructions <sub><sup><sup>_b1e32d3_</sup></sup></sub>

* introduce optional cap on visible tabs for horizontal and vertical positions <sub><sup><sup>_1dc837c_</sup></sup></sub>

* update <sub><sup><sup>_37b2697_</sup></sup></sub>

### 📝 Other

* Merge pull request #29 (via ComfyGit) <sub><sup><sup>_b6383ef_</sup></sup></sub>

---

## Changelog `v0.5.9` <sup><div align="end">🗓️ 2026-06-07</div></sup>

### 💥 💥 💥 This Release's Top Picks ...  💥 💥 💥

<sup>💬 Intro:</sup>  
<sup>_Again, quite big functional improvements. GIF showing new features attached. All features well documented._</sup>  

#### **1. &nbsp;&nbsp;&nbsp;Full tab-bar alignment support!**
- `start`|`end`|`center`
- available in both modes `horizontal`|`vertical`
- enhanced rendering

#### **2. &nbsp;&nbsp;&nbsp;New positioning logic!**
- Horizontal mode enhanced with `Bottom` position
- Vertical with `Right`
- fully adjusted rendering to all positions

#### **3. &nbsp;&nbsp;&nbsp;Scrolling indicator overhaul!**
- Unified scroll indication
- `OverflowPolicy::Scroll` is now default
  - make sure to apply `Truncate` if it fits better your use case 

#### **4. &nbsp;&nbsp;&nbsp;Lowering minimum Rust version from 1.95 to 1.88**
- follows the current Ratatui MSRV
- compile test passes on 1.88 


<sub>...  🎉 Enjoy!</sub>

<br>

### 💫 _Changed in:_ **tests**

#### 🐛 Fix(es)

* correct expected symbols for trailing junctions in demo tests to ensure accurate rendering validation <sub><sup><sup>_ad5a5fc_</sup></sup></sub>

---

## 💬 General Improvements & Fixes:

### 🧩 Features

* add HorizontalPosition and VerticalPosition enums for tab and rail placement <sub><sup><sup>_14c377f_</sup></sup></sub>

* implement horizontal and vertical origin functions for tab and rail positioning <sub><sup><sup>_021df9e_</sup></sup></sub>

* add VerticalPosition enum to configuration exports <sub><sup><sup>_a5da517_</sup></sup></sub>

* enhance TabNav with horizontal and vertical positioning options <sub><sup><sup>_2583145_</sup></sup></sub>

* refine rendering logic for horizontal and vertical tabs with improved baseline calculations <sub><sup><sup>_965fb6a_</sup></sup></sub>

* implement dynamic horizontal and vertical positioning for tabs in demo application <sub><sup><sup>_e250e6d_</sup></sup></sub>

* enhance content border rendering for horizontal and vertical modes in demo application <sub><sup><sup>_ba637fa_</sup></sup></sub>

* add TabBarAlign enum for tab strip alignment in horizontal and vertical modes <sub><sup><sup>_7e9e85c_</sup></sup></sub>

* implement tab entry building logic for forward and backward navigation in tab bar <sub><sup><sup>_d35b3e4_</sup></sup></sub>

* add tab_bar_align property to TabNav for customizable tab strip alignment <sub><sup><sup>_6f6b6d4_</sup></sup></sub>

* implement tab alignment toggle functionality in demo application <sub><sup><sup>_6bd2985_</sup></sup></sub>

* refactor tab bar end cap logic for horizontal and vertical rendering <sub><sup><sup>_4279988_</sup></sup></sub>

* add group_bounds method to TabViewport for calculating visible tab span <sub><sup><sup>_7c7e73c_</sup></sup></sub>

* enhance horizontal tab rendering with trailing junctions and margin caps for improved visual continuity <sub><sup><sup>_9ab6690_</sup></sup></sub>

* add scroll overflow markers for horizontal and vertical tab navigation to enhance user experience <sub><sup><sup>_b32e171_</sup></sup></sub>

* add demo GIF for version 0.5.9 showcasing new features <sub><sup><sup>_e1231f6_</sup></sup></sub>

### 🐛 Fix(es)

* add conditional rendering for bottom borders in horizontal tab layout <sub><sup><sup>_7beb155_</sup></sup></sub>

* update TabBarAlign reference in public exports for consistency <sub><sup><sup>_08daca8_</sup></sup></sub>

* correct vertical cap mirroring function to ensure proper alignment in tab bar rendering <sub><sup><sup>_500fc29_</sup></sup></sub>

* improve scroll offset logic in tab navigation to ensure selected tab visibility <sub><sup><sup>_f590347_</sup></sup></sub>

* restore default attribute for Scroll variant in OverflowPolicy enum to ensure correct behavior in tab navigation <sub><sup><sup>_5a6ebaa_</sup></sup></sub>

### 🔧 Maintenance

* CG app version bump to v0.5.0 <sub><sup><sup>_891149b_</sup></sup></sub>

* CG app version bump to v0.5.1 <sub><sup><sup>_c43b075_</sup></sup></sub>

* CG app version bump to v0.5.2 <sub><sup><sup>_d694fcb_</sup></sup></sub>

* CG app version bump to v0.5.3 <sub><sup><sup>_43c1cd5_</sup></sup></sub>

* CG app version bump to v0.5.4 <sub><sup><sup>_fe8c566_</sup></sup></sub>

* lowering MSRV to 1.88 in Cargo.toml <sub><sup><sup>_8deaf0d_</sup></sup></sub>

* CG app version bump to v0.5.5 <sub><sup><sup>_2635297_</sup></sup></sub>

* CG app version bump to v0.5.6 <sub><sup><sup>_98f0d6b_</sup></sup></sub>

* CG app version bump to v0.5.7 <sub><sup><sup>_1b3ea96_</sup></sup></sub>

* CG app version bump to v0.5.8 <sub><sup><sup>_2ae9a59_</sup></sup></sub>

* CG app version bump to v0.5.9 <sub><sup><sup>_a9a76b3_</sup></sup></sub>

### ℹ️ Documentation

* enhance documentation for TabBarEnd enum with detailed glyph descriptions for end caps <sub><sup><sup>_7bd2d3f_</sup></sup></sub>

* update documentation for TabBarEnd enum to clarify cap glyph behavior based on TabBarAlign <sub><sup><sup>_abfc3c1_</sup></sup></sub>

* update overflow policy documentation to clarify scroll affordances and truncate behavior in tab navigation <sub><sup><sup>_1fd5980_</sup></sup></sub>

* update overflow policy documentation to reflect default behavior change from Truncate to Scroll in tab navigation <sub><sup><sup>_888cb08_</sup></sup></sub>

* update README to highlight new features in v0.5, including strip positioning, alignment options, rendering fixes, and default scroll overflow behavior <sub><sup><sup>_eafecfc_</sup></sup></sub>

* add demo GIF for version 0.5.9 to README <sub><sup><sup>_9761c93_</sup></sup></sub>

### ♻️ Refactor

* simplify content border rendering logic for horizontal and vertical modes in demo application <sub><sup><sup>_b564aaf_</sup></sup></sub>

* enhance tab bar end cap rendering logic to support alignment options and viewport visibility <sub><sup><sup>_9486694_</sup></sup></sub>

* improve alignment logic in tab bar rendering for better handling of flow and group spans <sub><sup><sup>_42ce446_</sup></sup></sub>

* encapsulate tab bar end rendering parameters in structs for improved readability and maintainability <sub><sup><sup>_ea3e29e_</sup></sup></sub>

* simplify rendering logic for inactive horizontal tabs by removing unnecessary parameters and conditions <sub><sup><sup>_0f2472f_</sup></sup></sub>

* enhance alignment logic in horizontal tab bar rendering by introducing conditional cap alignment based on exact fit of group bounds <sub><sup><sup>_d27a46d_</sup></sup></sub>

* rename and update vertical cap mirroring function <sub><sup><sup>_0472e94_</sup></sup></sub>

* streamline vertical tab bar rendering by separating leading and trailing junction logic and enhancing margin cap handling <sub><sup><sup>_3ce273f_</sup></sup></sub>

* enhance vertical tab bar rendering by consolidating junction and margin cap logic for improved clarity and maintainability <sub><sup><sup>_6b5be0c_</sup></sup></sub>

* fixes for vertical exact fit on the right <sub><sup><sup>_d131e59_</sup></sup></sub>

* remove unused scroll affordance logic and simplify viewport calculations in tab navigation <sub><sup><sup>_ff903fb_</sup></sup></sub>

* simplify group_bounds and compute_viewport functions for improved readability in tab navigation <sub><sup><sup>_694ac30_</sup></sup></sub>

* streamline conditional statements and add clippy linting for argument count in rendering functions <sub><sup><sup>_289c78c_</sup></sup></sub>

* consolidate tab bar end cap functions and improve margin handling for horizontal tab rendering <sub><sup><sup>_3410454_</sup></sup></sub>

### 🧪 Tests

* add unit tests for horizontal and vertical tab positioning and active tab behavior <sub><sup><sup>_9f8cb0f_</sup></sup></sub>

* add assertions for bottom border rendering in horizontal tab layout <sub><sup><sup>_277a77d_</sup></sup></sub>

* add unit tests for horizontal and vertical tab alignment in TabNav <sub><sup><sup>_95458be_</sup></sup></sub>

* add unit tests for square and rounded tab bar end caps in horizontal and vertical orientations <sub><sup><sup>_09ecea3_</sup></sup></sub>

* add unit tests for center alignment of tab bar end caps in horizontal and vertical orientations <sub><sup><sup>_c2e6a91_</sup></sup></sub>

* add unit tests for horizontal and vertical end tab rendering to ensure no overlap and proper scrolling behavior <sub><sup><sup>_712fd72_</sup></sup></sub>

* add comprehensive demo tests for tab navigation width and alignment to validate rendering behavior <sub><sup><sup>_3086cfc_</sup></sup></sub>

* add assertions for expected symbols in demo tests to validate rendering of trailing caps <sub><sup><sup>_4ca8479_</sup></sup></sub>

* add new test for vertical tab bar end alignment with random right position and correct expected symbols <sub><sup><sup>_6df4f3c_</sup></sup></sub>

* update assertions for vertical tab bar end symbols and add new tests for exact fit scenarios <sub><sup><sup>_bd1dc39_</sup></sup></sub>

* update vertical tab bar end assertions and add new tests for right position scenarios <sub><sup><sup>_e9a9f19_</sup></sup></sub>

* add new tests for tab navigation scroll behavior and overflow markers <sub><sup><sup>_ab17666_</sup></sup></sub>

* update tab navigation tests to include overflow handling and ensure correct rendering behavior <sub><sup><sup>_066d528_</sup></sup></sub>

* add comprehensive tests for tab navigation rendering with various configurations and edge cases <sub><sup><sup>_2117b20_</sup></sup></sub>

### 📝 Other

* Merge pull request #19 (via ComfyGit) <sub><sup><sup>_0e75b7f_</sup></sup></sub>

* Merge pull request #20 (via ComfyGit) <sub><sup><sup>_8d756ef_</sup></sup></sub>

* Merge pull request #21 (via ComfyGit) <sub><sup><sup>_fe9b6cf_</sup></sup></sub>

* Merge pull request #22 (via ComfyGit) <sub><sup><sup>_1a81081_</sup></sup></sub>

* Merge pull request #23 (via ComfyGit) <sub><sup><sup>_b9aafe5_</sup></sup></sub>

* Merge pull request #24 (via ComfyGit) <sub><sup><sup>_c9559a0_</sup></sup></sub>

* Merge pull request #25 (via ComfyGit) <sub><sup><sup>_e22349a_</sup></sup></sub>

* Merge pull request #26 (via ComfyGit) <sub><sup><sup>_e329d6a_</sup></sup></sub>

* Merge pull request #27 (via ComfyGit) <sub><sup><sup>_fce465b_</sup></sup></sub>

* Merge pull request #28 (via ComfyGit) <sub><sup><sup>_d585f19_</sup></sup></sub>

---

## Changelog `v0.4.4` <sup><div align="end">🗓️ 2026-06-02</div></sup>

### 💥 💥 💥 This Release's Top Picks ...  💥 💥 💥

<sup>💬 Intro:</sup>  
<sup>_First of all, our apologies, developers! I forgot to exclude the demo GIF in Cargo.toml which caused the crate being over 7MB🤦 in v0.3.x... it is fixed now._</sup>  
<sup>_Apart from that, this release brings 2 exciting features that made my hair even greyer, but they both worked out very well in the end._</sup>  

#### **1. &nbsp;&nbsp;&nbsp;Selection FLASH Indication**
- The feature is well documented + I attached a separate GIF to highlight this feature + it's in DEMO example (feel free to examine), so just a few bullet points here:
  - R-C-Tabs now can be configured to highlight/indicate newly selected tab
  - This is done by a quick (~600ms) blink
  - Color is fully customizable

#### **2. &nbsp;&nbsp;&nbsp;Tab REORDERING**
- Again, the feature is well documented, and also included in the attached GIF+DEMO, to sum it up:
  - There are 3 master configs:
    - `NonePinned`: when selected, any tab can be moved to a new position
    - `SomePinned`: when selected, you can assign a pin to a tab, it's on per-tab basis which allows you to force any tab to keep its position while non-pinned tabs can be freely reorganized!
    - `AllPinned`: I did not want to introduce breaking change, so this is the default when undeclared. AllPined = non-moveable.
  - The feature has built-in highlight for the tab that's being dragged!


<sub>...  🎉 Enjoy!</sub>

<br>

### 🧩 Features

* add mouse tab reordering functionality and update tab selection logic <sub><sup><sup>_9df7319_</sup></sup></sub>

* introduce TabReorderPolicy enum to manage tab reordering behavior <sub><sup><sup>_9c2f63e_</sup></sup></sub>

* add reorder module and expose tab reordering functions <sub><sup><sup>_2414afd_</sup></sup></sub>

* enhance TabNav with reorder policy and mouse drag support <sub><sup><sup>_6fc4e36_</sup></sup></sub>

* implement tab reordering logic with drag-and-drop support <sub><sup><sup>_4b49a33_</sup></sup></sub>

* add mouse drag handling for tab reordering in TabNavState <sub><sup><sup>_63cc4c5_</sup></sup></sub>

* add reorder_drag_style to TabNav for customizable drag appearance <sub><sup><sup>_19151f2_</sup></sup></sub>

* enhance TabNav rendering to support tab reordering with visual feedback <sub><sup><sup>_36979ad_</sup></sup></sub>

* implement selection flash toggle and adjust event polling timeout in App <sub><sup><sup>_22910d2_</sup></sup></sub>

* expose selection flash constants in state module for improved tab navigation <sub><sup><sup>_319cfd7_</sup></sup></sub>

* add selection flash style and enable toggle in TabNav for enhanced user feedback <sub><sup><sup>_ca6bcfb_</sup></sup></sub>

* enhance TabNav rendering by integrating selection flash state into tab border styles for improved visual feedback <sub><sup><sup>_fb94185_</sup></sup></sub>

* implement selection flash functionality in TabNav for enhanced user interaction during tab selection changes <sub><sup><sup>_abc3183_</sup></sup></sub>

* add tab_entry_rect function to calculate layout for individual tabs based on orientation and available area <sub><sup><sup>_351698d_</sup></sup></sub>

* implement tab reordering with pinned tabs support and enhance index remapping functionality <sub><sup><sup>_81fbf52_</sup></sup></sub>

* add new demo GIFs for version 0.3 and 0.4 <sub><sup><sup>_9cef7dc_</sup></sup></sub>

### 🐛 Fix(es)

* update key binding for toggling selection flash in demo application from 'H' to 'F' <sub><sup><sup>_1591c56_</sup></sup></sub>

* update tab selection logic to support pinned tabs during reordering <sub><sup><sup>_14b5a2d_</sup></sup></sub>

* correct expected tab order in mouse reorder test for unpinned tabs <sub><sup><sup>_381441b_</sup></sup></sub>

* correct Git remote name casing in releaseNOW script <sub><sup><sup>_39cd376_</sup></sup></sub>

### 🔧 Maintenance

* update Cargo.toml to exclude GIF assets from the package - to fix the size problem <sub><sup><sup>_3d0724f_</sup></sup></sub>

* CG app version bump to v0.4.0 <sub><sup><sup>_bfeb414_</sup></sup></sub>

* CG app version bump to v0.4.1 <sub><sup><sup>_9736013_</sup></sup></sub>

* CG app version bump to v0.4.2 <sub><sup><sup>_f011811_</sup></sup></sub>

* CG app version bump to v0.4.3 <sub><sup><sup>_5756157_</sup></sup></sub>

* CG app version bump to v0.4.4 <sub><sup><sup>_6cea13d_</sup></sup></sub>

* remove demo gif <sub><sup><sup>_f725281_</sup></sup></sub>

### ℹ️ Documentation

* update README to include tab reordering features and configuration options <sub><sup><sup>_66f0ee8_</sup></sup></sub>

* update README to clarify drag reorder behavior and introduce selection flash details for TabNav <sub><sup><sup>_de07fed_</sup></sup></sub>

* update demo GIFs in README for versions 0.3 and 0.4 <sub><sup><sup>_c69e024_</sup></sup></sub>

### ♻️ Refactor

* extract content status text into a separate method for improved readability <sub><sup><sup>_6d4958e_</sup></sup></sub>

* simplify key handling for horizontal mode in demo application by removing redundant key mappings <sub><sup><sup>_2ef4c29_</sup></sup></sub>

* streamline tab label handling in demo application by consolidating label retrieval logic and improving scroll state management <sub><sup><sup>_8ded299_</sup></sup></sub>

* reorder module import to improve organization and clarity in lib.rs <sub><sup><sup>_a92781e_</sup></sup></sub>

* optimize tab entry retrieval logic in TabNav by utilizing tab_entry_rect for improved clarity and performance <sub><sup><sup>_b18e3a3_</sup></sup></sub>

* enhance code readability in render functions by simplifying match arms and formatting <sub><sup><sup>_a184536_</sup></sup></sub>

* simplify function signature of can_drag_index for improved readability <sub><sup><sup>_372fa79_</sup></sup></sub>

* improve code clarity by simplifying function calls and adding clear_scroll method to reset scroll offset <sub><sup><sup>_8a64264_</sup></sup></sub>

* update demo application to enhance tab pinning logic and improve command recording clarity <sub><sup><sup>_61b52d6_</sup></sup></sub>

* add remap_selected_index_with_pins function to reorder module for enhanced tab management <sub><sup><sup>_4c0c4ba_</sup></sup></sub>

### 🧪 Tests

* add unit test for mouse reordering of unpinned tabs in TabNavState <sub><sup><sup>_4691c45_</sup></sup></sub>

* add unit test for highlighting source tab during reorder drag with indexed color <sub><sup><sup>_e770307_</sup></sup></sub>

* add unit tests for unarmed drag highlighting and selection flash behavior in TabNav <sub><sup><sup>_c47e6bd_</sup></sup></sub>

* add vertical tab index tests to validate tab positioning and scroll behavior <sub><sup><sup>_4e83b30_</sup></sup></sub>

### 📝 Other

* Merge pull request #14 (via ComfyGit) <sub><sup><sup>_b9ac919_</sup></sup></sub>

* Merge pull request #15 (via ComfyGit) <sub><sup><sup>_7ba50e9_</sup></sup></sub>

* Merge pull request #16 (via ComfyGit) <sub><sup><sup>_5f8c280_</sup></sup></sub>

* Merge pull request #17 (via ComfyGit) <sub><sup><sup>_83f198c_</sup></sup></sub>

* Merge pull request #18 (via ComfyGit) <sub><sup><sup>_a606156_</sup></sup></sub>

---

## Changelog `v0.3.4` <sup><div align="end">🗓️ 2026-06-01</div></sup>

### 💥 💥 💥 This Release's Top Picks ...  💥 💥 💥

#### **1. &nbsp;&nbsp;&nbsp;Just a HOTFIX release.**
- Fixes wrong border render of the first tab when this tab is selected
- This bug affected only horizontal configuration
- SORTED!


<sub>...  🎉 Enjoy!</sub>

<br>

### 🐛 Fix(es)

* update horizontal tab bar rendering to reflect selection state in left cap <sub><sup><sup>_36af7c9_</sup></sup></sub>

### 🔧 Maintenance

* CG app version bump to v0.3.4 <sub><sup><sup>_985e95c_</sup></sup></sub>

### ℹ️ Documentation

* enhance documentation for TabBarEnd variants to clarify square and rounded cap behavior <sub><sup><sup>_051474b_</sup></sup></sub>

* update README.md with repository badges and improve TabBarEnd table formatting <sub><sup><sup>_3ac5cdc_</sup></sup></sub>

### 🧪 Tests

* add tests for horizontal tab bar rendering with selection state and inactive tabs <sub><sup><sup>_d2b5e88_</sup></sup></sub>

### 📝 Other

* Merge pull request #13 (via ComfyGit) <sub><sup><sup>_c45f44a_</sup></sup></sub>

---

## Changelog `v0.3.3` <sup><div align="end">🗓️ 2026-05-31</div></sup>

### 💥 💥 💥 This Release's Top Picks ...  💥 💥 💥

<sup>💬 Intro:</sup>  
<sup>_v0.3.3 is the very first public release of ratatui-comfy-tabs. Here's what it brings..._</sup>  

#### **1. &nbsp;&nbsp;&nbsp;Vertical tab rails — `TabOrientation::Vertical`, multi-line labels, and `vertical_label()` for stacked single-character rows; active tab opens toward content on the right.**
#### **2. &nbsp;&nbsp;&nbsp;Overflow that scales — `OverflowPolicy::Truncate` (default) or `Scroll` with `‹` / `›` / `…` affordances; `TabNavState::scroll_offset` drives a sliding window when tabs do not fit.**
#### **3. &nbsp;&nbsp;&nbsp;Geometry you can trust — `tab_rects()`, `tab_index_at()`, and `wheel_hover()` share the same layout math as rendering; optional `tab_widths()` / `tab_heights()` overrides fix hit-target drift (ComfyGit’s main pain point with `tui-tabs`).**
#### **4. &nbsp;&nbsp;&nbsp;Unicode-aware sizing — label width uses `unicode-width` display width (CJK and wide glyphs count correctly).**
#### **5. &nbsp;&nbsp;&nbsp;StatefulWidget + navigation — `TabNavState` with `select_direction`, `ensure_selected_visible`, `TabAxis` / `TabDirection` helpers, and keyboard-friendly scroll helpers.**
#### **6. &nbsp;&nbsp;&nbsp;Mouse input — wheel tab cycling (`handle_mouse_wheel`, touchpad axis mapping via `TabWheelDirection::from_axes`) and click-to-select (`handle_mouse_click`); both opt-out via `.mouse_wheel()` / `.mouse_click()`.**
#### **7. &nbsp;&nbsp;&nbsp;Layout polish — CSS-like `TabMargin` and `TabPadding`, `TabBarEnd` baseline caps (`NoEnd` / `Sqr` / `Rnd`), `tab_border::Rnd` or `tab_border::Sqr` via `border_set`, optional indicator, and orientation-specific defaults.**
#### **8. &nbsp;&nbsp;&nbsp;Production-ready crate — split modules (`config`, `nav`, `state`, `layout`, `render`), 44+ tests, interactive `demo` example, `ratatui-core` only (no terminal backend in the library).**

<sub>...  🎉 Enjoy!</sub>

<br>

### 🧩 Features

* enhance demo application with new tab navigation features including overflow handling and tab width toggling <sub><sup><sup>_fa3948a_</sup></sup></sub>

* introduce tab navigation state management with overflow handling and directional selection capabilities <sub><sup><sup>_9147066_</sup></sup></sub>

* add mouse wheel support and enhance default app configuration in demo application <sub><sup><sup>_1833939_</sup></sup></sub>

* implement tab configuration structures including margins, padding, orientation, and overflow policies <sub><sup><sup>_499c86a_</sup></sup></sub>

* add vertical label conversion function to transform single-line text into a vertical stack <sub><sup><sup>_3bb8c4c_</sup></sup></sub>

* implement tab layout management with effective margins, padding, and viewport calculations <sub><sup><sup>_42ad5f4_</sup></sup></sub>

* add TabNav structure for enhanced tab navigation with customizable styles, overflow policies, and state management <sub><sup><sup>_7bfdbe8_</sup></sup></sub>

* implement TabNav rendering logic with horizontal and vertical layouts, including overflow handling and customizable styles <sub><sup><sup>_ff3030b_</sup></sup></sub>

* introduce TabNavState for managing tab selection and scroll state, enhancing tab navigation functionality <sub><sup><sup>_74f4059_</sup></sup></sub>

* add mouse wheel tab switching functionality to TabNavState, enhancing user navigation experience <sub><sup><sup>_6e17ecf_</sup></sup></sub>

* enhance mouse wheel navigation in demo app by implementing mouse capture and refined handling for horizontal and vertical tab scrolling <sub><sup><sup>_ad6810d_</sup></sup></sub>

* add from_axes method to TabWheelDirection for improved tab switching based on scroll orientation <sub><sup><sup>_4515edb_</sup></sup></sub>

* implement wheel_hover method in TabNav for improved mouse wheel tab switching functionality <sub><sup><sup>_2ba7482_</sup></sup></sub>

* enhance demo app functionality by adding mouse click handling and command recording for user interactions <sub><sup><sup>_0abc168_</sup></sup></sub>

* add mouse click support for tab selection in TabNav, enabling user interaction with visible tabs <sub><sup><sup>_fe6876c_</sup></sup></sub>

* implement handle_mouse_click method in TabNavState for tab selection via mouse clicks, enhancing user interaction with tab navigation <sub><sup><sup>_9d3550b_</sup></sup></sub>

* introduce unified border-set names for TabNav with Rnd and Sqr aliases <sub><sup><sup>_e821bdc_</sup></sup></sub>

### 🔧 Maintenance

* CG app version bump to v0.3.0 <sub><sup><sup>_9b804a4_</sup></sup></sub>

* add unicode-width dependency to enhance text handling in tab navigation <sub><sup><sup>_464f772_</sup></sup></sub>

* update license information and enhance README description for clarity <sub><sup><sup>_377222b_</sup></sup></sub>

* CG app version bump to v0.3.1 <sub><sup><sup>_7ecb7fd_</sup></sup></sub>

* CG app version bump to v0.3.2 <sub><sup><sup>_5e9ba13_</sup></sup></sub>

* CG app version bump to v0.3.3 <sub><sup><sup>_ff9ea02_</sup></sup></sub>

* update demo.gif to reflect recent design changes <sub><sup><sup>_9032cd1_</sup></sup></sub>

* replace demo.gif with updated version to align with recent design modifications <sub><sup><sup>_58faaba_</sup></sup></sub>

* update demo.gif <sub><sup><sup>_0f1c768_</sup></sup></sub>

* remove dupl license from Cargo.toml to streamline project configuration <sub><sup><sup>_096a342_</sup></sup></sub>

### ℹ️ Documentation

* add overflow handling and Unicode-aware label width to tab navigation <sub><sup><sup>_dac1350_</sup></sup></sub>

* update README to clarify mouse wheel event handling and provide code examples for improved tab navigation functionality <sub><sup><sup>_47908ce_</sup></sup></sub>

* update README to include mouse click tab selection details and usage examples for TabNavState <sub><sup><sup>_ed6033e_</sup></sup></sub>

* expand README with detailed crate roadmap and usage instructions for TabNav features <sub><sup><sup>_d05f7ea_</sup></sup></sub>

* update README to reflect changes in border_set and tab_bar_end options for TabNav <sub><sup><sup>_254f3ad_</sup></sup></sub>

### 🎨 Visuals

* refine code formatting and improve readability in demo application, particularly in tab navigation logic <sub><sup><sup>_fd4cb12_</sup></sup></sub>

### ♻️ Refactor

* streamline tab management structures by removing unused components and optimizing layout definitions <sub><sup><sup>_57119e4_</sup></sup></sub>

* simplify mouse position checking in TabNavState by replacing Position with wheel_hover method for improved clarity and functionality <sub><sup><sup>_2a0bf7e_</sup></sup></sub>

* clean up formatting and improve readability in demo.rs by adjusting line breaks and spacing <sub><sup><sup>_ece6a45_</sup></sup></sub>

* reorganize imports in render.rs for improved clarity and consistency <sub><sup><sup>_d71d7e7_</sup></sup></sub>

* reorganize and simplify test functions in tests.rs for better readability and maintainability <sub><sup><sup>_60edeb2_</sup></sup></sub>

* enhance footer segment handling in demo.rs by introducing new functions for wrapping and segment management <sub><sup><sup>_d491c24_</sup></sup></sub>

* simplify border and tab bar end handling in demo.rs by renaming enum variants for clarity <sub><sup><sup>_74bd1c3_</sup></sup></sub>

* rename enum variant in TabBarEnd for improved clarity <sub><sup><sup>_8270627_</sup></sup></sub>

* update border_set in TabNav to use crate::tab_border::Rnd for consistency <sub><sup><sup>_e1238be_</sup></sup></sub>

* update TabBarEnd variants to use Sqr for horizontal and vertical tab bar ends <sub><sup><sup>_311ef0a_</sup></sup></sub>

* update TabNav to use Sqr for border_set and TabBarEnd in tests <sub><sup><sup>_7c9b2eb_</sup></sup></sub>

### 🧪 Tests

* add comprehensive tests for TabNav rendering, margin, padding, and overflow behavior <sub><sup><sup>_ed0cf4f_</sup></sup></sub>

* add unit tests for TabWheelDirection and wheel_hover functionality to validate tab navigation behavior <sub><sup><sup>_bea7767_</sup></sup></sub>

* add unit tests for mouse click interactions in TabNav, verifying tab selection behavior and handling of disabled state <sub><sup><sup>_d50f7e2_</sup></sup></sub>

### 📝 Other

* Merge pull request #9 (via ComfyGit) <sub><sup><sup>_48ac9d7_</sup></sup></sub>

* Merge pull request #10 (via ComfyGit) <sub><sup><sup>_89a4139_</sup></sup></sub>

* Merge pull request #11 (via ComfyGit) <sub><sup><sup>_eb4f042_</sup></sup></sub>

* Merge pull request #12 (via ComfyGit) <sub><sup><sup>_eb8911b_</sup></sup></sub>

---

## Changelog `v0.2.3` <sup><div align="end">🗓️ 2026-05-31</div></sup>

### 🧩 Features

* add vertical tab navigation example using ratatui <sub><sup><sup>_50848ef_</sup></sup></sub>

* enhance TabNav with orientation support and vertical label conversion <sub><sup><sup>_2cbb2f9_</sup></sup></sub>

* implement interactive demo with horizontal and vertical tab navigation modes <sub><sup><sup>_978146a_</sup></sup></sub>

* enhance TabNav indicator behavior for vertical and horizontal orientations <sub><sup><sup>_97d7aec_</sup></sup></sub>

* improve demo UI with shortcut footer and content rendering enhancements <sub><sup><sup>_f0ba413_</sup></sup></sub>

* add TabMargin and TabPadding structs for customizable tab layout and spacing <sub><sup><sup>_0d501cc_</sup></sup></sub>

* add padding preset and tab bar end options to demo application <sub><sup><sup>_b9f0c0a_</sup></sup></sub>

* introduce TabBarEnd enum for customizable tab strip end caps and enhance TabNav with new tab_bar_end option <sub><sup><sup>_8e473ff_</sup></sup></sub>

* update demo application to use PaddingPreset enum and add all_caps functionality for tab navigation <sub><sup><sup>_3f72f74_</sup></sup></sub>

* add all_caps option to TabNav for rendering uppercase tab labels and update TabBarEnd enum documentation <sub><sup><sup>_1ac5393_</sup></sup></sub>

### 🐛 Fix(es)

* update TabNav import to use ratatui_comfy_tabs <sub><sup><sup>_8d9ac94_</sup></sup></sub>

* correct border symbol rendering for active and inactive states in the UI <sub><sup><sup>_979e77e_</sup></sup></sub>

### 🔧 Maintenance

* CG app version bump to v0.1.0 <sub><sup><sup>_07cc6d8_</sup></sup></sub>

* update rust-version to 1.95 in Cargo.toml <sub><sup><sup>_f9a8006_</sup></sup></sub>

* CG app version bump to v0.1.1 <sub><sup><sup>_dccec7b_</sup></sup></sub>

* CG app version bump to v0.1.2 <sub><sup><sup>_6f2149a_</sup></sup></sub>

* CG app version bump to v0.2.0 <sub><sup><sup>_7d72bcb_</sup></sup></sub>

* CG app version bump to v0.2.1 <sub><sup><sup>_8c544d5_</sup></sup></sub>

* CG app version bump to v0.2.2 <sub><sup><sup>_a5cf56e_</sup></sup></sub>

* CG app version bump to v0.2.3 <sub><sup><sup>_5dc0979_</sup></sup></sub>

### ℹ️ Documentation

* Add CONTRIBUTING.md <sub><sup><sup>_6be200c_</sup></sup></sub>

* Add LICENSE <sub><sup><sup>_00bedd9_</sup></sup></sub>

* update README <sub><sup><sup>_41ec3df_</sup></sup></sub>

* update README to clarify tab navigation features and usage examples <sub><sup><sup>_393ed2a_</sup></sup></sub>

* update README to include new shortcut for toggling border styles <sub><sup><sup>_b7b7f12_</sup></sup></sub>

* update README to reflect new TabMargin and TabPadding features, including usage examples <sub><sup><sup>_2591216_</sup></sup></sub>

* enhance README with new tab bar end options and update default padding values for tab layouts <sub><sup><sup>_d5f155e_</sup></sup></sub>

* update README to include all_caps option for tab labels and clarify padding preset descriptions <sub><sup><sup>_f777123_</sup></sup></sub>

* update README to include new tab sizing options and geometry details for TabNav <sub><sup><sup>_2e08359_</sup></sup></sub>

### 🎨 Visuals

* improve code formatting and consistency in tab rendering functions <sub><sup><sup>_0d8c3de_</sup></sup></sub>

### ♻️ Refactor

* remove deprecated example files in favor of a unified demo example <sub><sup><sup>_d05e9d1_</sup></sup></sub>

* simplify vertical rail width calculation and adjust layout for horizontal rendering <sub><sup><sup>_3750a0b_</sup></sup></sub>

* simplify tab definitions and improve layout formatting in demo application <sub><sup><sup>_21c0fa2_</sup></sup></sub>

### 📝 Other

* Merge pull request #1 (via ComfyGit) <sub><sup><sup>_d8d313d_</sup></sup></sub>

* Merge pull request #2 (via ComfyGit) <sub><sup><sup>_02d3c6f_</sup></sup></sub>

* Add releaseNOW script for streamlined Rust crate releases <sub><sup><sup>_13fd8d9_</sup></sup></sub>

* Update license information and copyright notice in lib.rs <sub><sup><sup>_dfab8b2_</sup></sup></sub>

* Merge remote-tracking branch 'gitlab/main' into HEAD <sub><sup><sup>_296bd54_</sup></sup></sub>

* Merge pull request #4 (via ComfyGit) <sub><sup><sup>_7c74e56_</sup></sup></sub>

* Merge pull request #5 (via ComfyGit) <sub><sup><sup>_e09f924_</sup></sup></sub>

* Merge pull request #6 (via ComfyGit) <sub><sup><sup>_9ebaa01_</sup></sup></sub>

* Merge pull request #7 (via ComfyGit) <sub><sup><sup>_89af767_</sup></sup></sub>

* Merge pull request #8 (via ComfyGit) <sub><sup><sup>_86eff59_</sup></sup></sub>



---
... ✨ made with [ComfyGit](https://github.com/comfy-home/ComfyGit)