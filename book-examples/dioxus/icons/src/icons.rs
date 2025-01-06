use dioxus::prelude::*;
use radix_dioxus_icons::*;
#[component]
pub fn Icons() -> Element {
    rsx! {
        div { class: "w-full max-w-80 py-4",
            IconsA {}
            IconsB {}
            IconsC {}
            IconsD {}
            IconsE {}
            IconsF {}
            IconsG {}
            IconsH {}
            IconsI {}
            IconsK {}
            IconsL {}
            IconsM {}
            IconsN {}
            IconsO {}
            IconsP {}
            IconsQ {}
            IconsR {}
            IconsS {}
            IconsT {}
            IconsU {}
            IconsV {}
            IconsW {}
            IconsZ {}
        }
    }
}
#[component]
pub fn IconsA() -> Element {
    let icons = [
        (
            rsx! {
                AccessibilityIcon {}
            },
            "Accessibility",
        ),
        (
            rsx! {
                ActivityLogIcon {}
            },
            "Activity Log",
        ),
        (
            rsx! {
                AlignBaselineIcon {}
            },
            "Align Baseline",
        ),
        (
            rsx! {
                AlignBottomIcon {}
            },
            "Align Bottom",
        ),
        (
            rsx! {
                AlignCenterHorizontallyIcon {}
            },
            "Align Center Horizontally",
        ),
        (
            rsx! {
                AlignCenterVerticallyIcon {}
            },
            "Align Center Vertically",
        ),
        (
            rsx! {
                AlignLeftIcon {}
            },
            "Align Left",
        ),
        (
            rsx! {
                AlignRightIcon {}
            },
            "Align Right",
        ),
        (
            rsx! {
                AlignTopIcon {}
            },
            "Align Top",
        ),
        (
            rsx! {
                AllSidesIcon {}
            },
            "All Sides",
        ),
        (
            rsx! {
                AngleIcon {}
            },
            "Angle",
        ),
        (
            rsx! {
                ArchiveIcon {}
            },
            "Archive",
        ),
        (
            rsx! {
                ArrowBottomLeftIcon {}
            },
            "Arrow Bottom Left",
        ),
        (
            rsx! {
                ArrowBottomRightIcon {}
            },
            "Arrow Bottom Right",
        ),
        (
            rsx! {
                ArrowDownIcon {}
            },
            "Arrow Down",
        ),
        (
            rsx! {
                ArrowLeftIcon {}
            },
            "Arrow Left",
        ),
        (
            rsx! {
                ArrowRightIcon {}
            },
            "Arrow Right",
        ),
        (
            rsx! {
                ArrowTopLeftIcon {}
            },
            "Arrow Top Left",
        ),
        (
            rsx! {
                ArrowTopRightIcon {}
            },
            "Arrow Top Right",
        ),
        (
            rsx! {
                ArrowUpIcon {}
            },
            "Arrow Up",
        ),
        (
            rsx! {
                AspectRatioIcon {}
            },
            "Aspect Ratio",
        ),
        (
            rsx! {
                AvatarIcon {}
            },
            "Avatar",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsB() -> Element {
    let icons = [
        (
            rsx! {
                BackpackIcon {}
            },
            "Backpack",
        ),
        (
            rsx! {
                BadgeIcon {}
            },
            "Badge",
        ),
        (
            rsx! {
                BarChartIcon {}
            },
            "Bar Chart",
        ),
        (
            rsx! {
                BellIcon {}
            },
            "Bell",
        ),
        (
            rsx! {
                BlendingModeIcon {}
            },
            "Blending Mode",
        ),
        (
            rsx! {
                BookmarkIcon {}
            },
            "Bookmark",
        ),
        (
            rsx! {
                BookmarkFilledIcon {}
            },
            "Bookmark Filled",
        ),
        (
            rsx! {
                BorderAllIcon {}
            },
            "Border All",
        ),
        (
            rsx! {
                BorderBottomIcon {}
            },
            "Border Bottom",
        ),
        (
            rsx! {
                BorderDashedIcon {}
            },
            "Border Dashed",
        ),
        (
            rsx! {
                BorderDottedIcon {}
            },
            "Border Dotted",
        ),
        (
            rsx! {
                BorderLeftIcon {}
            },
            "Border Left",
        ),
        (
            rsx! {
                BorderNoneIcon {}
            },
            "Border None",
        ),
        (
            rsx! {
                BorderRightIcon {}
            },
            "Border Right",
        ),
        (
            rsx! {
                BorderSolidIcon {}
            },
            "Border Solid",
        ),
        (
            rsx! {
                BorderSplitIcon {}
            },
            "Border Split",
        ),
        (
            rsx! {
                BorderStyleIcon {}
            },
            "Border Style",
        ),
        (
            rsx! {
                BorderTopIcon {}
            },
            "Border Top",
        ),
        (
            rsx! {
                BorderWidthIcon {}
            },
            "Border Width",
        ),
        (
            rsx! {
                BoxIcon {}
            },
            "Box",
        ),
        (
            rsx! {
                BoxModelIcon {}
            },
            "Box Model",
        ),
        (
            rsx! {
                ButtonIcon {}
            },
            "Button",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsC() -> Element {
    let icons = [
        (
            rsx! {
                CalendarIcon {}
            },
            "Calendar",
        ),
        (
            rsx! {
                CameraIcon {}
            },
            "Camera",
        ),
        (
            rsx! {
                CardStackIcon {}
            },
            "Card Stack",
        ),
        (
            rsx! {
                CardStackMinusIcon {}
            },
            "Card Stack Minus",
        ),
        (
            rsx! {
                CardStackPlusIcon {}
            },
            "Card Stack Plus",
        ),
        (
            rsx! {
                CaretDownIcon {}
            },
            "Caret Down",
        ),
        (
            rsx! {
                CaretLeftIcon {}
            },
            "Caret Left",
        ),
        (
            rsx! {
                CaretRightIcon {}
            },
            "Caret Right",
        ),
        (
            rsx! {
                CaretSortIcon {}
            },
            "Caret Sort",
        ),
        (
            rsx! {
                CaretUpIcon {}
            },
            "Caret Up",
        ),
        (
            rsx! {
                ChatBubbleIcon {}
            },
            "Chat Bubble",
        ),
        (
            rsx! {
                CheckIcon {}
            },
            "Check",
        ),
        (
            rsx! {
                CheckCircledIcon {}
            },
            "Check Circled",
        ),
        (
            rsx! {
                CheckboxIcon {}
            },
            "Checkbox",
        ),
        (
            rsx! {
                ChevronDownIcon {}
            },
            "Chevron Down",
        ),
        (
            rsx! {
                ChevronLeftIcon {}
            },
            "Chevron Left",
        ),
        (
            rsx! {
                ChevronRightIcon {}
            },
            "Chevron Right",
        ),
        (
            rsx! {
                ChevronUpIcon {}
            },
            "Chevron Up",
        ),
        (
            rsx! {
                CircleIcon {}
            },
            "Circle",
        ),
        (
            rsx! {
                CircleBackslashIcon {}
            },
            "Circle Backslash",
        ),
        (
            rsx! {
                ClipboardIcon {}
            },
            "Clipboard",
        ),
        (
            rsx! {
                ClipboardCopyIcon {}
            },
            "Clipboard Copy",
        ),
        (
            rsx! {
                ClockIcon {}
            },
            "Clock",
        ),
        (
            rsx! {
                CodeIcon {}
            },
            "Code",
        ),
        (
            rsx! {
                CodesandboxLogoIcon {}
            },
            "Codesandbox Logo",
        ),
        (
            rsx! {
                ColorWheelIcon {}
            },
            "Color Wheel",
        ),
        (
            rsx! {
                ColumnSpacingIcon {}
            },
            "Column Spacing",
        ),
        (
            rsx! {
                ColumnsIcon {}
            },
            "Columns",
        ),
        (
            rsx! {
                CommitIcon {}
            },
            "Commit",
        ),
        (
            rsx! {
                Component1Icon {}
            },
            "Component 1",
        ),
        (
            rsx! {
                Component2Icon {}
            },
            "Component 2",
        ),
        (
            rsx! {
                ComponentBooleanIcon {}
            },
            "Component Boolean",
        ),
        (
            rsx! {
                ComponentInstanceIcon {}
            },
            "Component Instance",
        ),
        (
            rsx! {
                ComponentNoneIcon {}
            },
            "Component None",
        ),
        (
            rsx! {
                ComponentPlaceholderIcon {}
            },
            "Component Placeholder",
        ),
        (
            rsx! {
                ContainerIcon {}
            },
            "Container",
        ),
        (
            rsx! {
                CookieIcon {}
            },
            "Cookie",
        ),
        (
            rsx! {
                CopyIcon {}
            },
            "Copy",
        ),
        (
            rsx! {
                CornerBottomLeftIcon {}
            },
            "Corner Bottom Left",
        ),
        (
            rsx! {
                CornerBottomRightIcon {}
            },
            "Corner Bottom Right",
        ),
        (
            rsx! {
                CornerTopLeftIcon {}
            },
            "Corner Top Left",
        ),
        (
            rsx! {
                CornerTopRightIcon {}
            },
            "Corner Top Right",
        ),
        (
            rsx! {
                CornersIcon {}
            },
            "Corners",
        ),
        (
            rsx! {
                CountdownTimerIcon {}
            },
            "Countdown Timer",
        ),
        (
            rsx! {
                CounterClockwiseClockIcon {}
            },
            "Counter Clockwise Clock",
        ),
        (
            rsx! {
                CropIcon {}
            },
            "Crop",
        ),
        (
            rsx! {
                Cross1Icon {}
            },
            "Cross 1",
        ),
        (
            rsx! {
                Cross2Icon {}
            },
            "Cross 2",
        ),
        (
            rsx! {
                CrossCircledIcon {}
            },
            "Cross Circled",
        ),
        (
            rsx! {
                Crosshair1Icon {}
            },
            "Crosshair 1",
        ),
        (
            rsx! {
                Crosshair2Icon {}
            },
            "Crosshair 2",
        ),
        (
            rsx! {
                CrumpledPaperIcon {}
            },
            "Crumpled Paper",
        ),
        (
            rsx! {
                CubeIcon {}
            },
            "Cube",
        ),
        (
            rsx! {
                CursorArrowIcon {}
            },
            "Cursor Arrow",
        ),
        (
            rsx! {
                CursorTextIcon {}
            },
            "Cursor Text",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsD() -> Element {
    let icons = [
        (
            rsx! {
                DashIcon {}
            },
            "Dash",
        ),
        (
            rsx! {
                DashboardIcon {}
            },
            "Dashboard",
        ),
        (
            rsx! {
                DesktopIcon {}
            },
            "Desktop",
        ),
        (
            rsx! {
                DimensionsIcon {}
            },
            "Dimensions",
        ),
        (
            rsx! {
                DiscIcon {}
            },
            "Disc",
        ),
        (
            rsx! {
                DiscordLogoIcon {}
            },
            "Discord Logo",
        ),
        (
            rsx! {
                DividerHorizontalIcon {}
            },
            "Divider Horizontal",
        ),
        (
            rsx! {
                DividerVerticalIcon {}
            },
            "Divider Vertical",
        ),
        (
            rsx! {
                DotIcon {}
            },
            "Dot",
        ),
        (
            rsx! {
                DotFilledIcon {}
            },
            "Dot Filled",
        ),
        (
            rsx! {
                DotsHorizontalIcon {}
            },
            "Dots Horizontal",
        ),
        (
            rsx! {
                DotsVerticalIcon {}
            },
            "Dots Vertical",
        ),
        (
            rsx! {
                DoubleArrowDownIcon {}
            },
            "Double Arrow Down",
        ),
        (
            rsx! {
                DoubleArrowLeftIcon {}
            },
            "Double Arrow Left",
        ),
        (
            rsx! {
                DoubleArrowRightIcon {}
            },
            "Double Arrow Right",
        ),
        (
            rsx! {
                DoubleArrowUpIcon {}
            },
            "Double Arrow Up",
        ),
        (
            rsx! {
                DownloadIcon {}
            },
            "Download",
        ),
        (
            rsx! {
                DragHandleDots1Icon {}
            },
            "Drag Handle Dots 1",
        ),
        (
            rsx! {
                DragHandleDots2Icon {}
            },
            "Drag Handle Dots 2",
        ),
        (
            rsx! {
                DragHandleHorizontalIcon {}
            },
            "Drag Handle Horizontal",
        ),
        (
            rsx! {
                DragHandleVerticalIcon {}
            },
            "Drag Handle Vertical",
        ),
        (
            rsx! {
                DrawingPinIcon {}
            },
            "Drawing Pin",
        ),
        (
            rsx! {
                DrawingPinFilledIcon {}
            },
            "Drawing Pin Filled",
        ),
        (
            rsx! {
                DropdownMenuIcon {}
            },
            "Dropdown Menu",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsE() -> Element {
    let icons = [
        (
            rsx! {
                EnterIcon {}
            },
            "Enter",
        ),
        (
            rsx! {
                EnterFullScreenIcon {}
            },
            "Enter Full Screen",
        ),
        (
            rsx! {
                EnvelopeClosedIcon {}
            },
            "Envelope Closed",
        ),
        (
            rsx! {
                EnvelopeOpenIcon {}
            },
            "Envelope Open",
        ),
        (
            rsx! {
                EraserIcon {}
            },
            "Eraser",
        ),
        (
            rsx! {
                ExclamationTriangleIcon {}
            },
            "Exclamation Triangle",
        ),
        (
            rsx! {
                ExitIcon {}
            },
            "Exit",
        ),
        (
            rsx! {
                ExitFullScreenIcon {}
            },
            "Exit Full Screen",
        ),
        (
            rsx! {
                ExternalLinkIcon {}
            },
            "External Link",
        ),
        (
            rsx! {
                EyeClosedIcon {}
            },
            "Eye Closed",
        ),
        (
            rsx! {
                EyeNoneIcon {}
            },
            "Eye None",
        ),
        (
            rsx! {
                EyeOpenIcon {}
            },
            "Eye Open",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsF() -> Element {
    let icons = [
        (
            rsx! {
                FaceIcon {}
            },
            "Face",
        ),
        (
            rsx! {
                FigmaLogoIcon {}
            },
            "Figma Logo",
        ),
        (
            rsx! {
                FileIcon {}
            },
            "File",
        ),
        (
            rsx! {
                FileMinusIcon {}
            },
            "File Minus",
        ),
        (
            rsx! {
                FilePlusIcon {}
            },
            "File Plus",
        ),
        (
            rsx! {
                FileTextIcon {}
            },
            "File Text",
        ),
        (
            rsx! {
                FontBoldIcon {}
            },
            "Font Bold",
        ),
        (
            rsx! {
                FontFamilyIcon {}
            },
            "Font Family",
        ),
        (
            rsx! {
                FontItalicIcon {}
            },
            "Font Italic",
        ),
        (
            rsx! {
                FontRomanIcon {}
            },
            "Font Roman",
        ),
        (
            rsx! {
                FontSizeIcon {}
            },
            "Font Size",
        ),
        (
            rsx! {
                FontStyleIcon {}
            },
            "Font Style",
        ),
        (
            rsx! {
                FrameIcon {}
            },
            "Frame",
        ),
        (
            rsx! {
                FramerLogoIcon {}
            },
            "Framer Logo",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsG() -> Element {
    let icons = [
        (
            rsx! {
                GearIcon {}
            },
            "Gear",
        ),
        (
            rsx! {
                GithubLogoIcon {}
            },
            "Github Logo",
        ),
        (
            rsx! {
                GlobeIcon {}
            },
            "Globe",
        ),
        (
            rsx! {
                GridIcon {}
            },
            "Grid",
        ),
        (
            rsx! {
                GroupIcon {}
            },
            "Group",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsH() -> Element {
    let icons = [
        (
            rsx! {
                Half1Icon {}
            },
            "Half 1",
        ),
        (
            rsx! {
                Half2Icon {}
            },
            "Half 2",
        ),
        (
            rsx! {
                HamburgerMenuIcon {}
            },
            "Hamburger Menu",
        ),
        (
            rsx! {
                HandIcon {}
            },
            "Hand",
        ),
        (
            rsx! {
                HeadingIcon {}
            },
            "Heading",
        ),
        (
            rsx! {
                HeartIcon {}
            },
            "Heart",
        ),
        (
            rsx! {
                HeartFilledIcon {}
            },
            "Heart Filled",
        ),
        (
            rsx! {
                HeightIcon {}
            },
            "Height",
        ),
        (
            rsx! {
                HobbyKnifeIcon {}
            },
            "Hobby Knife",
        ),
        (
            rsx! {
                HomeIcon {}
            },
            "Home",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsI() -> Element {
    let icons = [
        (
            rsx! {
                IconjarLogoIcon {}
            },
            "Iconjar Logo",
        ),
        (
            rsx! {
                IdCardIcon {}
            },
            "Id Card",
        ),
        (
            rsx! {
                ImageIcon {}
            },
            "Image",
        ),
        (
            rsx! {
                InfoCircledIcon {}
            },
            "Info Circled",
        ),
        (
            rsx! {
                InputIcon {}
            },
            "Input",
        ),
        (
            rsx! {
                InstagramLogoIcon {}
            },
            "Instagram Logo",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsK() -> Element {
    let icons = [(
        rsx! {
            KeyboardIcon {}
        },
        "Keyboard",
    )];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsL() -> Element {
    let icons = [
        (
            rsx! {
                LapTimerIcon {}
            },
            "Lap Timer",
        ),
        (
            rsx! {
                LaptopIcon {}
            },
            "Laptop",
        ),
        (
            rsx! {
                LayersIcon {}
            },
            "Layers",
        ),
        (
            rsx! {
                LayoutIcon {}
            },
            "Layout",
        ),
        (
            rsx! {
                LetterCaseCapitalizeIcon {}
            },
            "Letter Case Capitalize",
        ),
        (
            rsx! {
                LetterCaseLowercaseIcon {}
            },
            "Letter Case Lowercase",
        ),
        (
            rsx! {
                LetterCaseToggleIcon {}
            },
            "Letter Case Toggle",
        ),
        (
            rsx! {
                LetterCaseUppercaseIcon {}
            },
            "Letter Case Uppercase",
        ),
        (
            rsx! {
                LetterSpacingIcon {}
            },
            "Letter Spacing",
        ),
        (
            rsx! {
                LightningBoltIcon {}
            },
            "Lightning Bolt",
        ),
        (
            rsx! {
                LineHeightIcon {}
            },
            "Line Height",
        ),
        (
            rsx! {
                Link1Icon {}
            },
            "Link 1",
        ),
        (
            rsx! {
                Link2Icon {}
            },
            "Link 2",
        ),
        (
            rsx! {
                LinkBreak1Icon {}
            },
            "Link Break 1",
        ),
        (
            rsx! {
                LinkBreak2Icon {}
            },
            "Link Break 2",
        ),
        (
            rsx! {
                LinkNone1Icon {}
            },
            "Link None 1",
        ),
        (
            rsx! {
                LinkNone2Icon {}
            },
            "Link None 2",
        ),
        (
            rsx! {
                LinkedinLogoIcon {}
            },
            "Linkedin Logo",
        ),
        (
            rsx! {
                ListBulletIcon {}
            },
            "List Bullet",
        ),
        (
            rsx! {
                LockClosedIcon {}
            },
            "Lock Closed",
        ),
        (
            rsx! {
                LockOpen1Icon {}
            },
            "Lock Open 1",
        ),
        (
            rsx! {
                LockOpen2Icon {}
            },
            "Lock Open 2",
        ),
        (
            rsx! {
                LoopIcon {}
            },
            "Loop",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsM() -> Element {
    let icons = [
        (
            rsx! {
                MagicWandIcon {}
            },
            "Magic Wand",
        ),
        (
            rsx! {
                MagnifyingGlassIcon {}
            },
            "Magnifying Glass",
        ),
        (
            rsx! {
                MarginIcon {}
            },
            "Margin",
        ),
        (
            rsx! {
                MaskOffIcon {}
            },
            "Mask Off",
        ),
        (
            rsx! {
                MaskOnIcon {}
            },
            "Mask On",
        ),
        (
            rsx! {
                MinusIcon {}
            },
            "Minus",
        ),
        (
            rsx! {
                MinusCircledIcon {}
            },
            "Minus Circled",
        ),
        (
            rsx! {
                MixIcon {}
            },
            "Mix",
        ),
        (
            rsx! {
                MixerHorizontalIcon {}
            },
            "Mixer Horizontal",
        ),
        (
            rsx! {
                MixerVerticalIcon {}
            },
            "Mixer Vertical",
        ),
        (
            rsx! {
                MobileIcon {}
            },
            "Mobile",
        ),
        (
            rsx! {
                ModulzLogoIcon {}
            },
            "Modulz Logo",
        ),
        (
            rsx! {
                MoonIcon {}
            },
            "Moon",
        ),
        (
            rsx! {
                MoveIcon {}
            },
            "Move",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsN() -> Element {
    let icons = [(
        rsx! {
            NotionLogoIcon {}
        },
        "Notion Logo",
    )];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsO() -> Element {
    let icons = [
        (
            rsx! {
                OpacityIcon {}
            },
            "Opacity",
        ),
        (
            rsx! {
                OpenInNewWindowIcon {}
            },
            "Open In New Window",
        ),
        (
            rsx! {
                OverlineIcon {}
            },
            "Overline",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsP() -> Element {
    let icons = [
        (
            rsx! {
                PaddingIcon {}
            },
            "Padding",
        ),
        (
            rsx! {
                PaperPlaneIcon {}
            },
            "Paper Plane",
        ),
        (
            rsx! {
                PauseIcon {}
            },
            "Pause",
        ),
        (
            rsx! {
                Pencil1Icon {}
            },
            "Pencil 1",
        ),
        (
            rsx! {
                Pencil2Icon {}
            },
            "Pencil 2",
        ),
        (
            rsx! {
                PersonIcon {}
            },
            "Person",
        ),
        (
            rsx! {
                PieChartIcon {}
            },
            "Pie Chart",
        ),
        (
            rsx! {
                PilcrowIcon {}
            },
            "Pilcrow",
        ),
        (
            rsx! {
                PinBottomIcon {}
            },
            "Pin Bottom",
        ),
        (
            rsx! {
                PinLeftIcon {}
            },
            "Pin Left",
        ),
        (
            rsx! {
                PinRightIcon {}
            },
            "Pin Right",
        ),
        (
            rsx! {
                PinTopIcon {}
            },
            "Pin Top",
        ),
        (
            rsx! {
                PlayIcon {}
            },
            "Play",
        ),
        (
            rsx! {
                PlusIcon {}
            },
            "Plus",
        ),
        (
            rsx! {
                PlusCircledIcon {}
            },
            "Plus Circled",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsQ() -> Element {
    let icons = [
        (
            rsx! {
                QuestionMarkIcon {}
            },
            "Question Mark",
        ),
        (
            rsx! {
                QuestionMarkCircledIcon {}
            },
            "Question Mark Circled",
        ),
        (
            rsx! {
                QuoteIcon {}
            },
            "Quote",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsR() -> Element {
    let icons = [
        (
            rsx! {
                RadiobuttonIcon {}
            },
            "Radiobutton",
        ),
        (
            rsx! {
                ReaderIcon {}
            },
            "Reader",
        ),
        (
            rsx! {
                ReloadIcon {}
            },
            "Reload",
        ),
        (
            rsx! {
                ResetIcon {}
            },
            "Reset",
        ),
        (
            rsx! {
                ResumeIcon {}
            },
            "Resume",
        ),
        (
            rsx! {
                RocketIcon {}
            },
            "Rocket",
        ),
        (
            rsx! {
                RotateCounterClockwiseIcon {}
            },
            "Rotate Counter Clockwise",
        ),
        (
            rsx! {
                RowSpacingIcon {}
            },
            "Row Spacing",
        ),
        (
            rsx! {
                RowsIcon {}
            },
            "Rows",
        ),
        (
            rsx! {
                RulerHorizontalIcon {}
            },
            "Ruler Horizontal",
        ),
        (
            rsx! {
                RulerSquareIcon {}
            },
            "Ruler Square",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsS() -> Element {
    let icons = [
        (
            rsx! {
                ScissorsIcon {}
            },
            "Scissors",
        ),
        (
            rsx! {
                SectionIcon {}
            },
            "Section",
        ),
        (
            rsx! {
                SewingPinIcon {}
            },
            "Sewing Pin",
        ),
        (
            rsx! {
                SewingPinFilledIcon {}
            },
            "Sewing Pin Filled",
        ),
        (
            rsx! {
                ShadowIcon {}
            },
            "Shadow",
        ),
        (
            rsx! {
                ShadowInnerIcon {}
            },
            "Shadow Inner",
        ),
        (
            rsx! {
                ShadowNoneIcon {}
            },
            "Shadow None",
        ),
        (
            rsx! {
                ShadowOuterIcon {}
            },
            "Shadow Outer",
        ),
        (
            rsx! {
                Share1Icon {}
            },
            "Share 1",
        ),
        (
            rsx! {
                Share2Icon {}
            },
            "Share 2",
        ),
        (
            rsx! {
                ShuffleIcon {}
            },
            "Shuffle",
        ),
        (
            rsx! {
                SizeIcon {}
            },
            "Size",
        ),
        (
            rsx! {
                SketchLogoIcon {}
            },
            "Sketch Logo",
        ),
        (
            rsx! {
                SlashIcon {}
            },
            "Slash",
        ),
        (
            rsx! {
                SliderIcon {}
            },
            "Slider",
        ),
        (
            rsx! {
                SpaceBetweenHorizontallyIcon {}
            },
            "Space Between Horizontally",
        ),
        (
            rsx! {
                SpaceBetweenVerticallyIcon {}
            },
            "Space Between Vertically",
        ),
        (
            rsx! {
                SpaceEvenlyHorizontallyIcon {}
            },
            "Space Evenly Horizontally",
        ),
        (
            rsx! {
                SpaceEvenlyVerticallyIcon {}
            },
            "Space Evenly Vertically",
        ),
        (
            rsx! {
                SpeakerLoudIcon {}
            },
            "Speaker Loud",
        ),
        (
            rsx! {
                SpeakerModerateIcon {}
            },
            "Speaker Moderate",
        ),
        (
            rsx! {
                SpeakerOffIcon {}
            },
            "Speaker Off",
        ),
        (
            rsx! {
                SpeakerQuietIcon {}
            },
            "Speaker Quiet",
        ),
        (
            rsx! {
                SquareIcon {}
            },
            "Square",
        ),
        (
            rsx! {
                StackIcon {}
            },
            "Stack",
        ),
        (
            rsx! {
                StarIcon {}
            },
            "Star",
        ),
        (
            rsx! {
                StarFilledIcon {}
            },
            "Star Filled",
        ),
        (
            rsx! {
                StitchesLogoIcon {}
            },
            "Stitches Logo",
        ),
        (
            rsx! {
                StopIcon {}
            },
            "Stop",
        ),
        (
            rsx! {
                StopwatchIcon {}
            },
            "Stopwatch",
        ),
        (
            rsx! {
                StretchHorizontallyIcon {}
            },
            "Stretch Horizontally",
        ),
        (
            rsx! {
                StretchVerticallyIcon {}
            },
            "Stretch Vertically",
        ),
        (
            rsx! {
                StrikethroughIcon {}
            },
            "Strikethrough",
        ),
        (
            rsx! {
                SunIcon {}
            },
            "Sun",
        ),
        (
            rsx! {
                SwitchIcon {}
            },
            "Switch",
        ),
        (
            rsx! {
                SymbolIcon {}
            },
            "Symbol",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsT() -> Element {
    let icons = [
        (
            rsx! {
                TableIcon {}
            },
            "Table",
        ),
        (
            rsx! {
                TargetIcon {}
            },
            "Target",
        ),
        (
            rsx! {
                TextIcon {}
            },
            "Text",
        ),
        (
            rsx! {
                TextAlignBottomIcon {}
            },
            "Text Align Bottom",
        ),
        (
            rsx! {
                TextAlignCenterIcon {}
            },
            "Text Align Center",
        ),
        (
            rsx! {
                TextAlignJustifyIcon {}
            },
            "Text Align Justify",
        ),
        (
            rsx! {
                TextAlignLeftIcon {}
            },
            "Text Align Left",
        ),
        (
            rsx! {
                TextAlignMiddleIcon {}
            },
            "Text Align Middle",
        ),
        (
            rsx! {
                TextAlignRightIcon {}
            },
            "Text Align Right",
        ),
        (
            rsx! {
                TextAlignTopIcon {}
            },
            "Text Align Top",
        ),
        (
            rsx! {
                TextNoneIcon {}
            },
            "Text None",
        ),
        (
            rsx! {
                ThickArrowDownIcon {}
            },
            "Thick Arrow Down",
        ),
        (
            rsx! {
                ThickArrowLeftIcon {}
            },
            "Thick Arrow Left",
        ),
        (
            rsx! {
                ThickArrowRightIcon {}
            },
            "Thick Arrow Right",
        ),
        (
            rsx! {
                ThickArrowUpIcon {}
            },
            "Thick Arrow Up",
        ),
        (
            rsx! {
                TimerIcon {}
            },
            "Timer",
        ),
        (
            rsx! {
                TokensIcon {}
            },
            "Tokens",
        ),
        (
            rsx! {
                TrackNextIcon {}
            },
            "Track Next",
        ),
        (
            rsx! {
                TrackPreviousIcon {}
            },
            "Track Previous",
        ),
        (
            rsx! {
                TransformIcon {}
            },
            "Transform",
        ),
        (
            rsx! {
                TransparencyGridIcon {}
            },
            "Transparency Grid",
        ),
        (
            rsx! {
                TrashIcon {}
            },
            "Trash",
        ),
        (
            rsx! {
                TriangleDownIcon {}
            },
            "Triangle Down",
        ),
        (
            rsx! {
                TriangleLeftIcon {}
            },
            "Triangle Left",
        ),
        (
            rsx! {
                TriangleRightIcon {}
            },
            "Triangle Right",
        ),
        (
            rsx! {
                TriangleUpIcon {}
            },
            "Triangle Up",
        ),
        (
            rsx! {
                TwitterLogoIcon {}
            },
            "Twitter Logo",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsU() -> Element {
    let icons = [
        (
            rsx! {
                UnderlineIcon {}
            },
            "Underline",
        ),
        (
            rsx! {
                UpdateIcon {}
            },
            "Update",
        ),
        (
            rsx! {
                UploadIcon {}
            },
            "Upload",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsV() -> Element {
    let icons = [
        (
            rsx! {
                ValueIcon {}
            },
            "Value",
        ),
        (
            rsx! {
                ValueNoneIcon {}
            },
            "Value None",
        ),
        (
            rsx! {
                VercelLogoIcon {}
            },
            "Vercel Logo",
        ),
        (
            rsx! {
                VideoIcon {}
            },
            "Video",
        ),
        (
            rsx! {
                ViewGridIcon {}
            },
            "View Grid",
        ),
        (
            rsx! {
                ViewHorizontalIcon {}
            },
            "View Horizontal",
        ),
        (
            rsx! {
                ViewNoneIcon {}
            },
            "View None",
        ),
        (
            rsx! {
                ViewVerticalIcon {}
            },
            "View Vertical",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsW() -> Element {
    let icons = [(
        rsx! {
            WidthIcon {}
        },
        "Width",
    )];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsZ() -> Element {
    let icons = [
        (
            rsx! {
                ZoomInIcon {}
            },
            "Zoom In",
        ),
        (
            rsx! {
                ZoomOutIcon {}
            },
            "Zoom Out",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm text-white",
                {icon}
                span { {name} }
            }
        }
    }
}
