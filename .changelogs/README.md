# Changelog History

Newest archived changelogs first. When multiple archived files represent the same version, only the newest archive is included here.

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