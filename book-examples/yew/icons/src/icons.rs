use radix_yew_icons::*;
use yew::prelude::*;
#[function_component]
pub fn Icons() -> Html {
    html! {
        <div class="w-full max-w-80 py-4">
            <IconsA />
            <IconsB />
            <IconsC />
            <IconsD />
            <IconsE />
            <IconsF />
            <IconsG />
            <IconsH />
            <IconsI />
            <IconsK />
            <IconsL />
            <IconsM />
            <IconsN />
            <IconsO />
            <IconsP />
            <IconsQ />
            <IconsR />
            <IconsS />
            <IconsT />
            <IconsU />
            <IconsV />
            <IconsW />
            <IconsZ />
        </div>
    }
}
#[function_component]
pub fn IconsA() -> Html {
    let icons = [
        (html! { <AccessibilityIcon /> }, "Accessibility"),
        (html! { <ActivityLogIcon /> }, "Activity Log"),
        (html! { <AlignBaselineIcon /> }, "Align Baseline"),
        (html! { <AlignBottomIcon /> }, "Align Bottom"),
        (
            html! { <AlignCenterHorizontallyIcon /> },
            "Align Center Horizontally",
        ),
        (
            html! { <AlignCenterVerticallyIcon /> },
            "Align Center Vertically",
        ),
        (html! { <AlignLeftIcon /> }, "Align Left"),
        (html! { <AlignRightIcon /> }, "Align Right"),
        (html! { <AlignTopIcon /> }, "Align Top"),
        (html! { <AllSidesIcon /> }, "All Sides"),
        (html! { <AngleIcon /> }, "Angle"),
        (html! { <ArchiveIcon /> }, "Archive"),
        (html! { <ArrowBottomLeftIcon /> }, "Arrow Bottom Left"),
        (html! { <ArrowBottomRightIcon /> }, "Arrow Bottom Right"),
        (html! { <ArrowDownIcon /> }, "Arrow Down"),
        (html! { <ArrowLeftIcon /> }, "Arrow Left"),
        (html! { <ArrowRightIcon /> }, "Arrow Right"),
        (html! { <ArrowTopLeftIcon /> }, "Arrow Top Left"),
        (html! { <ArrowTopRightIcon /> }, "Arrow Top Right"),
        (html! { <ArrowUpIcon /> }, "Arrow Up"),
        (html! { <AspectRatioIcon /> }, "Aspect Ratio"),
        (html! { <AvatarIcon /> }, "Avatar"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsB() -> Html {
    let icons = [
        (html! { <BackpackIcon /> }, "Backpack"),
        (html! { <BadgeIcon /> }, "Badge"),
        (html! { <BarChartIcon /> }, "Bar Chart"),
        (html! { <BellIcon /> }, "Bell"),
        (html! { <BlendingModeIcon /> }, "Blending Mode"),
        (html! { <BookmarkIcon /> }, "Bookmark"),
        (html! { <BookmarkFilledIcon /> }, "Bookmark Filled"),
        (html! { <BorderAllIcon /> }, "Border All"),
        (html! { <BorderBottomIcon /> }, "Border Bottom"),
        (html! { <BorderDashedIcon /> }, "Border Dashed"),
        (html! { <BorderDottedIcon /> }, "Border Dotted"),
        (html! { <BorderLeftIcon /> }, "Border Left"),
        (html! { <BorderNoneIcon /> }, "Border None"),
        (html! { <BorderRightIcon /> }, "Border Right"),
        (html! { <BorderSolidIcon /> }, "Border Solid"),
        (html! { <BorderSplitIcon /> }, "Border Split"),
        (html! { <BorderStyleIcon /> }, "Border Style"),
        (html! { <BorderTopIcon /> }, "Border Top"),
        (html! { <BorderWidthIcon /> }, "Border Width"),
        (html! { <BoxIcon /> }, "Box"),
        (html! { <BoxModelIcon /> }, "Box Model"),
        (html! { <ButtonIcon /> }, "Button"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsC() -> Html {
    let icons = [
        (html! { <CalendarIcon /> }, "Calendar"),
        (html! { <CameraIcon /> }, "Camera"),
        (html! { <CardStackIcon /> }, "Card Stack"),
        (html! { <CardStackMinusIcon /> }, "Card Stack Minus"),
        (html! { <CardStackPlusIcon /> }, "Card Stack Plus"),
        (html! { <CaretDownIcon /> }, "Caret Down"),
        (html! { <CaretLeftIcon /> }, "Caret Left"),
        (html! { <CaretRightIcon /> }, "Caret Right"),
        (html! { <CaretSortIcon /> }, "Caret Sort"),
        (html! { <CaretUpIcon /> }, "Caret Up"),
        (html! { <ChatBubbleIcon /> }, "Chat Bubble"),
        (html! { <CheckIcon /> }, "Check"),
        (html! { <CheckCircledIcon /> }, "Check Circled"),
        (html! { <CheckboxIcon /> }, "Checkbox"),
        (html! { <ChevronDownIcon /> }, "Chevron Down"),
        (html! { <ChevronLeftIcon /> }, "Chevron Left"),
        (html! { <ChevronRightIcon /> }, "Chevron Right"),
        (html! { <ChevronUpIcon /> }, "Chevron Up"),
        (html! { <CircleIcon /> }, "Circle"),
        (html! { <CircleBackslashIcon /> }, "Circle Backslash"),
        (html! { <ClipboardIcon /> }, "Clipboard"),
        (html! { <ClipboardCopyIcon /> }, "Clipboard Copy"),
        (html! { <ClockIcon /> }, "Clock"),
        (html! { <CodeIcon /> }, "Code"),
        (html! { <CodesandboxLogoIcon /> }, "Codesandbox Logo"),
        (html! { <ColorWheelIcon /> }, "Color Wheel"),
        (html! { <ColumnSpacingIcon /> }, "Column Spacing"),
        (html! { <ColumnsIcon /> }, "Columns"),
        (html! { <CommitIcon /> }, "Commit"),
        (html! { <Component1Icon /> }, "Component 1"),
        (html! { <Component2Icon /> }, "Component 2"),
        (html! { <ComponentBooleanIcon /> }, "Component Boolean"),
        (html! { <ComponentInstanceIcon /> }, "Component Instance"),
        (html! { <ComponentNoneIcon /> }, "Component None"),
        (
            html! { <ComponentPlaceholderIcon /> },
            "Component Placeholder",
        ),
        (html! { <ContainerIcon /> }, "Container"),
        (html! { <CookieIcon /> }, "Cookie"),
        (html! { <CopyIcon /> }, "Copy"),
        (html! { <CornerBottomLeftIcon /> }, "Corner Bottom Left"),
        (html! { <CornerBottomRightIcon /> }, "Corner Bottom Right"),
        (html! { <CornerTopLeftIcon /> }, "Corner Top Left"),
        (html! { <CornerTopRightIcon /> }, "Corner Top Right"),
        (html! { <CornersIcon /> }, "Corners"),
        (html! { <CountdownTimerIcon /> }, "Countdown Timer"),
        (
            html! { <CounterClockwiseClockIcon /> },
            "Counter Clockwise Clock",
        ),
        (html! { <CropIcon /> }, "Crop"),
        (html! { <Cross1Icon /> }, "Cross 1"),
        (html! { <Cross2Icon /> }, "Cross 2"),
        (html! { <CrossCircledIcon /> }, "Cross Circled"),
        (html! { <Crosshair1Icon /> }, "Crosshair 1"),
        (html! { <Crosshair2Icon /> }, "Crosshair 2"),
        (html! { <CrumpledPaperIcon /> }, "Crumpled Paper"),
        (html! { <CubeIcon /> }, "Cube"),
        (html! { <CursorArrowIcon /> }, "Cursor Arrow"),
        (html! { <CursorTextIcon /> }, "Cursor Text"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsD() -> Html {
    let icons = [
        (html! { <DashIcon /> }, "Dash"),
        (html! { <DashboardIcon /> }, "Dashboard"),
        (html! { <DesktopIcon /> }, "Desktop"),
        (html! { <DimensionsIcon /> }, "Dimensions"),
        (html! { <DiscIcon /> }, "Disc"),
        (html! { <DiscordLogoIcon /> }, "Discord Logo"),
        (html! { <DividerHorizontalIcon /> }, "Divider Horizontal"),
        (html! { <DividerVerticalIcon /> }, "Divider Vertical"),
        (html! { <DotIcon /> }, "Dot"),
        (html! { <DotFilledIcon /> }, "Dot Filled"),
        (html! { <DotsHorizontalIcon /> }, "Dots Horizontal"),
        (html! { <DotsVerticalIcon /> }, "Dots Vertical"),
        (html! { <DoubleArrowDownIcon /> }, "Double Arrow Down"),
        (html! { <DoubleArrowLeftIcon /> }, "Double Arrow Left"),
        (html! { <DoubleArrowRightIcon /> }, "Double Arrow Right"),
        (html! { <DoubleArrowUpIcon /> }, "Double Arrow Up"),
        (html! { <DownloadIcon /> }, "Download"),
        (html! { <DragHandleDots1Icon /> }, "Drag Handle Dots 1"),
        (html! { <DragHandleDots2Icon /> }, "Drag Handle Dots 2"),
        (
            html! { <DragHandleHorizontalIcon /> },
            "Drag Handle Horizontal",
        ),
        (html! { <DragHandleVerticalIcon /> }, "Drag Handle Vertical"),
        (html! { <DrawingPinIcon /> }, "Drawing Pin"),
        (html! { <DrawingPinFilledIcon /> }, "Drawing Pin Filled"),
        (html! { <DropdownMenuIcon /> }, "Dropdown Menu"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsE() -> Html {
    let icons = [
        (html! { <EnterIcon /> }, "Enter"),
        (html! { <EnterFullScreenIcon /> }, "Enter Full Screen"),
        (html! { <EnvelopeClosedIcon /> }, "Envelope Closed"),
        (html! { <EnvelopeOpenIcon /> }, "Envelope Open"),
        (html! { <EraserIcon /> }, "Eraser"),
        (
            html! { <ExclamationTriangleIcon /> },
            "Exclamation Triangle",
        ),
        (html! { <ExitIcon /> }, "Exit"),
        (html! { <ExitFullScreenIcon /> }, "Exit Full Screen"),
        (html! { <ExternalLinkIcon /> }, "External Link"),
        (html! { <EyeClosedIcon /> }, "Eye Closed"),
        (html! { <EyeNoneIcon /> }, "Eye None"),
        (html! { <EyeOpenIcon /> }, "Eye Open"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsF() -> Html {
    let icons = [
        (html! { <FaceIcon /> }, "Face"),
        (html! { <FigmaLogoIcon /> }, "Figma Logo"),
        (html! { <FileIcon /> }, "File"),
        (html! { <FileMinusIcon /> }, "File Minus"),
        (html! { <FilePlusIcon /> }, "File Plus"),
        (html! { <FileTextIcon /> }, "File Text"),
        (html! { <FontBoldIcon /> }, "Font Bold"),
        (html! { <FontFamilyIcon /> }, "Font Family"),
        (html! { <FontItalicIcon /> }, "Font Italic"),
        (html! { <FontRomanIcon /> }, "Font Roman"),
        (html! { <FontSizeIcon /> }, "Font Size"),
        (html! { <FontStyleIcon /> }, "Font Style"),
        (html! { <FrameIcon /> }, "Frame"),
        (html! { <FramerLogoIcon /> }, "Framer Logo"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsG() -> Html {
    let icons = [
        (html! { <GearIcon /> }, "Gear"),
        (html! { <GithubLogoIcon /> }, "Github Logo"),
        (html! { <GlobeIcon /> }, "Globe"),
        (html! { <GridIcon /> }, "Grid"),
        (html! { <GroupIcon /> }, "Group"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsH() -> Html {
    let icons = [
        (html! { <Half1Icon /> }, "Half 1"),
        (html! { <Half2Icon /> }, "Half 2"),
        (html! { <HamburgerMenuIcon /> }, "Hamburger Menu"),
        (html! { <HandIcon /> }, "Hand"),
        (html! { <HeadingIcon /> }, "Heading"),
        (html! { <HeartIcon /> }, "Heart"),
        (html! { <HeartFilledIcon /> }, "Heart Filled"),
        (html! { <HeightIcon /> }, "Height"),
        (html! { <HobbyKnifeIcon /> }, "Hobby Knife"),
        (html! { <HomeIcon /> }, "Home"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsI() -> Html {
    let icons = [
        (html! { <IconjarLogoIcon /> }, "Iconjar Logo"),
        (html! { <IdCardIcon /> }, "Id Card"),
        (html! { <ImageIcon /> }, "Image"),
        (html! { <InfoCircledIcon /> }, "Info Circled"),
        (html! { <InputIcon /> }, "Input"),
        (html! { <InstagramLogoIcon /> }, "Instagram Logo"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsK() -> Html {
    let icons = [(html! { <KeyboardIcon /> }, "Keyboard")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsL() -> Html {
    let icons = [
        (html! { <LapTimerIcon /> }, "Lap Timer"),
        (html! { <LaptopIcon /> }, "Laptop"),
        (html! { <LayersIcon /> }, "Layers"),
        (html! { <LayoutIcon /> }, "Layout"),
        (
            html! { <LetterCaseCapitalizeIcon /> },
            "Letter Case Capitalize",
        ),
        (
            html! { <LetterCaseLowercaseIcon /> },
            "Letter Case Lowercase",
        ),
        (html! { <LetterCaseToggleIcon /> }, "Letter Case Toggle"),
        (
            html! { <LetterCaseUppercaseIcon /> },
            "Letter Case Uppercase",
        ),
        (html! { <LetterSpacingIcon /> }, "Letter Spacing"),
        (html! { <LightningBoltIcon /> }, "Lightning Bolt"),
        (html! { <LineHeightIcon /> }, "Line Height"),
        (html! { <Link1Icon /> }, "Link 1"),
        (html! { <Link2Icon /> }, "Link 2"),
        (html! { <LinkBreak1Icon /> }, "Link Break 1"),
        (html! { <LinkBreak2Icon /> }, "Link Break 2"),
        (html! { <LinkNone1Icon /> }, "Link None 1"),
        (html! { <LinkNone2Icon /> }, "Link None 2"),
        (html! { <LinkedinLogoIcon /> }, "Linkedin Logo"),
        (html! { <ListBulletIcon /> }, "List Bullet"),
        (html! { <LockClosedIcon /> }, "Lock Closed"),
        (html! { <LockOpen1Icon /> }, "Lock Open 1"),
        (html! { <LockOpen2Icon /> }, "Lock Open 2"),
        (html! { <LoopIcon /> }, "Loop"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsM() -> Html {
    let icons = [
        (html! { <MagicWandIcon /> }, "Magic Wand"),
        (html! { <MagnifyingGlassIcon /> }, "Magnifying Glass"),
        (html! { <MarginIcon /> }, "Margin"),
        (html! { <MaskOffIcon /> }, "Mask Off"),
        (html! { <MaskOnIcon /> }, "Mask On"),
        (html! { <MinusIcon /> }, "Minus"),
        (html! { <MinusCircledIcon /> }, "Minus Circled"),
        (html! { <MixIcon /> }, "Mix"),
        (html! { <MixerHorizontalIcon /> }, "Mixer Horizontal"),
        (html! { <MixerVerticalIcon /> }, "Mixer Vertical"),
        (html! { <MobileIcon /> }, "Mobile"),
        (html! { <ModulzLogoIcon /> }, "Modulz Logo"),
        (html! { <MoonIcon /> }, "Moon"),
        (html! { <MoveIcon /> }, "Move"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsN() -> Html {
    let icons = [(html! { <NotionLogoIcon /> }, "Notion Logo")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsO() -> Html {
    let icons = [
        (html! { <OpacityIcon /> }, "Opacity"),
        (html! { <OpenInNewWindowIcon /> }, "Open In New Window"),
        (html! { <OverlineIcon /> }, "Overline"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsP() -> Html {
    let icons = [
        (html! { <PaddingIcon /> }, "Padding"),
        (html! { <PaperPlaneIcon /> }, "Paper Plane"),
        (html! { <PauseIcon /> }, "Pause"),
        (html! { <Pencil1Icon /> }, "Pencil 1"),
        (html! { <Pencil2Icon /> }, "Pencil 2"),
        (html! { <PersonIcon /> }, "Person"),
        (html! { <PieChartIcon /> }, "Pie Chart"),
        (html! { <PilcrowIcon /> }, "Pilcrow"),
        (html! { <PinBottomIcon /> }, "Pin Bottom"),
        (html! { <PinLeftIcon /> }, "Pin Left"),
        (html! { <PinRightIcon /> }, "Pin Right"),
        (html! { <PinTopIcon /> }, "Pin Top"),
        (html! { <PlayIcon /> }, "Play"),
        (html! { <PlusIcon /> }, "Plus"),
        (html! { <PlusCircledIcon /> }, "Plus Circled"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsQ() -> Html {
    let icons = [
        (html! { <QuestionMarkIcon /> }, "Question Mark"),
        (
            html! { <QuestionMarkCircledIcon /> },
            "Question Mark Circled",
        ),
        (html! { <QuoteIcon /> }, "Quote"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsR() -> Html {
    let icons = [
        (html! { <RadiobuttonIcon /> }, "Radiobutton"),
        (html! { <ReaderIcon /> }, "Reader"),
        (html! { <ReloadIcon /> }, "Reload"),
        (html! { <ResetIcon /> }, "Reset"),
        (html! { <ResumeIcon /> }, "Resume"),
        (html! { <RocketIcon /> }, "Rocket"),
        (
            html! { <RotateCounterClockwiseIcon /> },
            "Rotate Counter Clockwise",
        ),
        (html! { <RowSpacingIcon /> }, "Row Spacing"),
        (html! { <RowsIcon /> }, "Rows"),
        (html! { <RulerHorizontalIcon /> }, "Ruler Horizontal"),
        (html! { <RulerSquareIcon /> }, "Ruler Square"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsS() -> Html {
    let icons = [
        (html! { <ScissorsIcon /> }, "Scissors"),
        (html! { <SectionIcon /> }, "Section"),
        (html! { <SewingPinIcon /> }, "Sewing Pin"),
        (html! { <SewingPinFilledIcon /> }, "Sewing Pin Filled"),
        (html! { <ShadowIcon /> }, "Shadow"),
        (html! { <ShadowInnerIcon /> }, "Shadow Inner"),
        (html! { <ShadowNoneIcon /> }, "Shadow None"),
        (html! { <ShadowOuterIcon /> }, "Shadow Outer"),
        (html! { <Share1Icon /> }, "Share 1"),
        (html! { <Share2Icon /> }, "Share 2"),
        (html! { <ShuffleIcon /> }, "Shuffle"),
        (html! { <SizeIcon /> }, "Size"),
        (html! { <SketchLogoIcon /> }, "Sketch Logo"),
        (html! { <SlashIcon /> }, "Slash"),
        (html! { <SliderIcon /> }, "Slider"),
        (
            html! { <SpaceBetweenHorizontallyIcon /> },
            "Space Between Horizontally",
        ),
        (
            html! { <SpaceBetweenVerticallyIcon /> },
            "Space Between Vertically",
        ),
        (
            html! { <SpaceEvenlyHorizontallyIcon /> },
            "Space Evenly Horizontally",
        ),
        (
            html! { <SpaceEvenlyVerticallyIcon /> },
            "Space Evenly Vertically",
        ),
        (html! { <SpeakerLoudIcon /> }, "Speaker Loud"),
        (html! { <SpeakerModerateIcon /> }, "Speaker Moderate"),
        (html! { <SpeakerOffIcon /> }, "Speaker Off"),
        (html! { <SpeakerQuietIcon /> }, "Speaker Quiet"),
        (html! { <SquareIcon /> }, "Square"),
        (html! { <StackIcon /> }, "Stack"),
        (html! { <StarIcon /> }, "Star"),
        (html! { <StarFilledIcon /> }, "Star Filled"),
        (html! { <StitchesLogoIcon /> }, "Stitches Logo"),
        (html! { <StopIcon /> }, "Stop"),
        (html! { <StopwatchIcon /> }, "Stopwatch"),
        (
            html! { <StretchHorizontallyIcon /> },
            "Stretch Horizontally",
        ),
        (html! { <StretchVerticallyIcon /> }, "Stretch Vertically"),
        (html! { <StrikethroughIcon /> }, "Strikethrough"),
        (html! { <SunIcon /> }, "Sun"),
        (html! { <SwitchIcon /> }, "Switch"),
        (html! { <SymbolIcon /> }, "Symbol"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsT() -> Html {
    let icons = [
        (html! { <TableIcon /> }, "Table"),
        (html! { <TargetIcon /> }, "Target"),
        (html! { <TextIcon /> }, "Text"),
        (html! { <TextAlignBottomIcon /> }, "Text Align Bottom"),
        (html! { <TextAlignCenterIcon /> }, "Text Align Center"),
        (html! { <TextAlignJustifyIcon /> }, "Text Align Justify"),
        (html! { <TextAlignLeftIcon /> }, "Text Align Left"),
        (html! { <TextAlignMiddleIcon /> }, "Text Align Middle"),
        (html! { <TextAlignRightIcon /> }, "Text Align Right"),
        (html! { <TextAlignTopIcon /> }, "Text Align Top"),
        (html! { <TextNoneIcon /> }, "Text None"),
        (html! { <ThickArrowDownIcon /> }, "Thick Arrow Down"),
        (html! { <ThickArrowLeftIcon /> }, "Thick Arrow Left"),
        (html! { <ThickArrowRightIcon /> }, "Thick Arrow Right"),
        (html! { <ThickArrowUpIcon /> }, "Thick Arrow Up"),
        (html! { <TimerIcon /> }, "Timer"),
        (html! { <TokensIcon /> }, "Tokens"),
        (html! { <TrackNextIcon /> }, "Track Next"),
        (html! { <TrackPreviousIcon /> }, "Track Previous"),
        (html! { <TransformIcon /> }, "Transform"),
        (html! { <TransparencyGridIcon /> }, "Transparency Grid"),
        (html! { <TrashIcon /> }, "Trash"),
        (html! { <TriangleDownIcon /> }, "Triangle Down"),
        (html! { <TriangleLeftIcon /> }, "Triangle Left"),
        (html! { <TriangleRightIcon /> }, "Triangle Right"),
        (html! { <TriangleUpIcon /> }, "Triangle Up"),
        (html! { <TwitterLogoIcon /> }, "Twitter Logo"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsU() -> Html {
    let icons = [
        (html! { <UnderlineIcon /> }, "Underline"),
        (html! { <UpdateIcon /> }, "Update"),
        (html! { <UploadIcon /> }, "Upload"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsV() -> Html {
    let icons = [
        (html! { <ValueIcon /> }, "Value"),
        (html! { <ValueNoneIcon /> }, "Value None"),
        (html! { <VercelLogoIcon /> }, "Vercel Logo"),
        (html! { <VideoIcon /> }, "Video"),
        (html! { <ViewGridIcon /> }, "View Grid"),
        (html! { <ViewHorizontalIcon /> }, "View Horizontal"),
        (html! { <ViewNoneIcon /> }, "View None"),
        (html! { <ViewVerticalIcon /> }, "View Vertical"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsW() -> Html {
    let icons = [(html! { <WidthIcon /> }, "Width")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsZ() -> Html {
    let icons = [
        (html! { <ZoomInIcon /> }, "Zoom In"),
        (html! { <ZoomOutIcon /> }, "Zoom Out"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
