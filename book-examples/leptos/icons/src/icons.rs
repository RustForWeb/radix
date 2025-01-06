use leptos::prelude::*;
use radix_leptos_icons::*;
#[component]
pub fn Icons() -> impl IntoView {
    view! {
        <div class="w-full max-w-80 py-4">
            <IconsA/>
            <IconsB/>
            <IconsC/>
            <IconsD/>
            <IconsE/>
            <IconsF/>
            <IconsG/>
            <IconsH/>
            <IconsI/>
            <IconsK/>
            <IconsL/>
            <IconsM/>
            <IconsN/>
            <IconsO/>
            <IconsP/>
            <IconsQ/>
            <IconsR/>
            <IconsS/>
            <IconsT/>
            <IconsU/>
            <IconsV/>
            <IconsW/>
            <IconsZ/>
        </div>
    }
}
#[component]
pub fn IconsA() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <AccessibilityIcon/> }.into_any(), "Accessibility"),
                (view! { <ActivityLogIcon/> }.into_any(), "Activity Log"),
                (view! { <AlignBaselineIcon/> }.into_any(), "Align Baseline"),
                (view! { <AlignBottomIcon/> }.into_any(), "Align Bottom"),
                (view! { <AlignCenterHorizontallyIcon/> }.into_any(), "Align Center Horizontally"),
                (view! { <AlignCenterVerticallyIcon/> }.into_any(), "Align Center Vertically"),
                (view! { <AlignLeftIcon/> }.into_any(), "Align Left"),
                (view! { <AlignRightIcon/> }.into_any(), "Align Right"),
                (view! { <AlignTopIcon/> }.into_any(), "Align Top"),
                (view! { <AllSidesIcon/> }.into_any(), "All Sides"),
                (view! { <AngleIcon/> }.into_any(), "Angle"),
                (view! { <ArchiveIcon/> }.into_any(), "Archive"),
                (view! { <ArrowBottomLeftIcon/> }.into_any(), "Arrow Bottom Left"),
                (view! { <ArrowBottomRightIcon/> }.into_any(), "Arrow Bottom Right"),
                (view! { <ArrowDownIcon/> }.into_any(), "Arrow Down"),
                (view! { <ArrowLeftIcon/> }.into_any(), "Arrow Left"),
                (view! { <ArrowRightIcon/> }.into_any(), "Arrow Right"),
                (view! { <ArrowTopLeftIcon/> }.into_any(), "Arrow Top Left"),
                (view! { <ArrowTopRightIcon/> }.into_any(), "Arrow Top Right"),
                (view! { <ArrowUpIcon/> }.into_any(), "Arrow Up"),
                (view! { <AspectRatioIcon/> }.into_any(), "Aspect Ratio"),
                (view! { <AvatarIcon/> }.into_any(), "Avatar"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsB() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <BackpackIcon/> }.into_any(), "Backpack"),
                (view! { <BadgeIcon/> }.into_any(), "Badge"),
                (view! { <BarChartIcon/> }.into_any(), "Bar Chart"),
                (view! { <BellIcon/> }.into_any(), "Bell"),
                (view! { <BlendingModeIcon/> }.into_any(), "Blending Mode"),
                (view! { <BookmarkIcon/> }.into_any(), "Bookmark"),
                (view! { <BookmarkFilledIcon/> }.into_any(), "Bookmark Filled"),
                (view! { <BorderAllIcon/> }.into_any(), "Border All"),
                (view! { <BorderBottomIcon/> }.into_any(), "Border Bottom"),
                (view! { <BorderDashedIcon/> }.into_any(), "Border Dashed"),
                (view! { <BorderDottedIcon/> }.into_any(), "Border Dotted"),
                (view! { <BorderLeftIcon/> }.into_any(), "Border Left"),
                (view! { <BorderNoneIcon/> }.into_any(), "Border None"),
                (view! { <BorderRightIcon/> }.into_any(), "Border Right"),
                (view! { <BorderSolidIcon/> }.into_any(), "Border Solid"),
                (view! { <BorderSplitIcon/> }.into_any(), "Border Split"),
                (view! { <BorderStyleIcon/> }.into_any(), "Border Style"),
                (view! { <BorderTopIcon/> }.into_any(), "Border Top"),
                (view! { <BorderWidthIcon/> }.into_any(), "Border Width"),
                (view! { <BoxIcon/> }.into_any(), "Box"),
                (view! { <BoxModelIcon/> }.into_any(), "Box Model"),
                (view! { <ButtonIcon/> }.into_any(), "Button"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsC() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <CalendarIcon/> }.into_any(), "Calendar"),
                (view! { <CameraIcon/> }.into_any(), "Camera"),
                (view! { <CardStackIcon/> }.into_any(), "Card Stack"),
                (view! { <CardStackMinusIcon/> }.into_any(), "Card Stack Minus"),
                (view! { <CardStackPlusIcon/> }.into_any(), "Card Stack Plus"),
                (view! { <CaretDownIcon/> }.into_any(), "Caret Down"),
                (view! { <CaretLeftIcon/> }.into_any(), "Caret Left"),
                (view! { <CaretRightIcon/> }.into_any(), "Caret Right"),
                (view! { <CaretSortIcon/> }.into_any(), "Caret Sort"),
                (view! { <CaretUpIcon/> }.into_any(), "Caret Up"),
                (view! { <ChatBubbleIcon/> }.into_any(), "Chat Bubble"),
                (view! { <CheckIcon/> }.into_any(), "Check"),
                (view! { <CheckCircledIcon/> }.into_any(), "Check Circled"),
                (view! { <CheckboxIcon/> }.into_any(), "Checkbox"),
                (view! { <ChevronDownIcon/> }.into_any(), "Chevron Down"),
                (view! { <ChevronLeftIcon/> }.into_any(), "Chevron Left"),
                (view! { <ChevronRightIcon/> }.into_any(), "Chevron Right"),
                (view! { <ChevronUpIcon/> }.into_any(), "Chevron Up"),
                (view! { <CircleIcon/> }.into_any(), "Circle"),
                (view! { <CircleBackslashIcon/> }.into_any(), "Circle Backslash"),
                (view! { <ClipboardIcon/> }.into_any(), "Clipboard"),
                (view! { <ClipboardCopyIcon/> }.into_any(), "Clipboard Copy"),
                (view! { <ClockIcon/> }.into_any(), "Clock"),
                (view! { <CodeIcon/> }.into_any(), "Code"),
                (view! { <CodesandboxLogoIcon/> }.into_any(), "Codesandbox Logo"),
                (view! { <ColorWheelIcon/> }.into_any(), "Color Wheel"),
                (view! { <ColumnSpacingIcon/> }.into_any(), "Column Spacing"),
                (view! { <ColumnsIcon/> }.into_any(), "Columns"),
                (view! { <CommitIcon/> }.into_any(), "Commit"),
                (view! { <Component1Icon/> }.into_any(), "Component 1"),
                (view! { <Component2Icon/> }.into_any(), "Component 2"),
                (view! { <ComponentBooleanIcon/> }.into_any(), "Component Boolean"),
                (view! { <ComponentInstanceIcon/> }.into_any(), "Component Instance"),
                (view! { <ComponentNoneIcon/> }.into_any(), "Component None"),
                (view! { <ComponentPlaceholderIcon/> }.into_any(), "Component Placeholder"),
                (view! { <ContainerIcon/> }.into_any(), "Container"),
                (view! { <CookieIcon/> }.into_any(), "Cookie"),
                (view! { <CopyIcon/> }.into_any(), "Copy"),
                (view! { <CornerBottomLeftIcon/> }.into_any(), "Corner Bottom Left"),
                (view! { <CornerBottomRightIcon/> }.into_any(), "Corner Bottom Right"),
                (view! { <CornerTopLeftIcon/> }.into_any(), "Corner Top Left"),
                (view! { <CornerTopRightIcon/> }.into_any(), "Corner Top Right"),
                (view! { <CornersIcon/> }.into_any(), "Corners"),
                (view! { <CountdownTimerIcon/> }.into_any(), "Countdown Timer"),
                (view! { <CounterClockwiseClockIcon/> }.into_any(), "Counter Clockwise Clock"),
                (view! { <CropIcon/> }.into_any(), "Crop"),
                (view! { <Cross1Icon/> }.into_any(), "Cross 1"),
                (view! { <Cross2Icon/> }.into_any(), "Cross 2"),
                (view! { <CrossCircledIcon/> }.into_any(), "Cross Circled"),
                (view! { <Crosshair1Icon/> }.into_any(), "Crosshair 1"),
                (view! { <Crosshair2Icon/> }.into_any(), "Crosshair 2"),
                (view! { <CrumpledPaperIcon/> }.into_any(), "Crumpled Paper"),
                (view! { <CubeIcon/> }.into_any(), "Cube"),
                (view! { <CursorArrowIcon/> }.into_any(), "Cursor Arrow"),
                (view! { <CursorTextIcon/> }.into_any(), "Cursor Text"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsD() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <DashIcon/> }.into_any(), "Dash"),
                (view! { <DashboardIcon/> }.into_any(), "Dashboard"),
                (view! { <DesktopIcon/> }.into_any(), "Desktop"),
                (view! { <DimensionsIcon/> }.into_any(), "Dimensions"),
                (view! { <DiscIcon/> }.into_any(), "Disc"),
                (view! { <DiscordLogoIcon/> }.into_any(), "Discord Logo"),
                (view! { <DividerHorizontalIcon/> }.into_any(), "Divider Horizontal"),
                (view! { <DividerVerticalIcon/> }.into_any(), "Divider Vertical"),
                (view! { <DotIcon/> }.into_any(), "Dot"),
                (view! { <DotFilledIcon/> }.into_any(), "Dot Filled"),
                (view! { <DotsHorizontalIcon/> }.into_any(), "Dots Horizontal"),
                (view! { <DotsVerticalIcon/> }.into_any(), "Dots Vertical"),
                (view! { <DoubleArrowDownIcon/> }.into_any(), "Double Arrow Down"),
                (view! { <DoubleArrowLeftIcon/> }.into_any(), "Double Arrow Left"),
                (view! { <DoubleArrowRightIcon/> }.into_any(), "Double Arrow Right"),
                (view! { <DoubleArrowUpIcon/> }.into_any(), "Double Arrow Up"),
                (view! { <DownloadIcon/> }.into_any(), "Download"),
                (view! { <DragHandleDots1Icon/> }.into_any(), "Drag Handle Dots 1"),
                (view! { <DragHandleDots2Icon/> }.into_any(), "Drag Handle Dots 2"),
                (view! { <DragHandleHorizontalIcon/> }.into_any(), "Drag Handle Horizontal"),
                (view! { <DragHandleVerticalIcon/> }.into_any(), "Drag Handle Vertical"),
                (view! { <DrawingPinIcon/> }.into_any(), "Drawing Pin"),
                (view! { <DrawingPinFilledIcon/> }.into_any(), "Drawing Pin Filled"),
                (view! { <DropdownMenuIcon/> }.into_any(), "Dropdown Menu"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsE() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <EnterIcon/> }.into_any(), "Enter"),
                (view! { <EnterFullScreenIcon/> }.into_any(), "Enter Full Screen"),
                (view! { <EnvelopeClosedIcon/> }.into_any(), "Envelope Closed"),
                (view! { <EnvelopeOpenIcon/> }.into_any(), "Envelope Open"),
                (view! { <EraserIcon/> }.into_any(), "Eraser"),
                (view! { <ExclamationTriangleIcon/> }.into_any(), "Exclamation Triangle"),
                (view! { <ExitIcon/> }.into_any(), "Exit"),
                (view! { <ExitFullScreenIcon/> }.into_any(), "Exit Full Screen"),
                (view! { <ExternalLinkIcon/> }.into_any(), "External Link"),
                (view! { <EyeClosedIcon/> }.into_any(), "Eye Closed"),
                (view! { <EyeNoneIcon/> }.into_any(), "Eye None"),
                (view! { <EyeOpenIcon/> }.into_any(), "Eye Open"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsF() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <FaceIcon/> }.into_any(), "Face"),
                (view! { <FigmaLogoIcon/> }.into_any(), "Figma Logo"),
                (view! { <FileIcon/> }.into_any(), "File"),
                (view! { <FileMinusIcon/> }.into_any(), "File Minus"),
                (view! { <FilePlusIcon/> }.into_any(), "File Plus"),
                (view! { <FileTextIcon/> }.into_any(), "File Text"),
                (view! { <FontBoldIcon/> }.into_any(), "Font Bold"),
                (view! { <FontFamilyIcon/> }.into_any(), "Font Family"),
                (view! { <FontItalicIcon/> }.into_any(), "Font Italic"),
                (view! { <FontRomanIcon/> }.into_any(), "Font Roman"),
                (view! { <FontSizeIcon/> }.into_any(), "Font Size"),
                (view! { <FontStyleIcon/> }.into_any(), "Font Style"),
                (view! { <FrameIcon/> }.into_any(), "Frame"),
                (view! { <FramerLogoIcon/> }.into_any(), "Framer Logo"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsG() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <GearIcon/> }.into_any(), "Gear"),
                (view! { <GithubLogoIcon/> }.into_any(), "Github Logo"),
                (view! { <GlobeIcon/> }.into_any(), "Globe"),
                (view! { <GridIcon/> }.into_any(), "Grid"),
                (view! { <GroupIcon/> }.into_any(), "Group"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsH() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Half1Icon/> }.into_any(), "Half 1"),
                (view! { <Half2Icon/> }.into_any(), "Half 2"),
                (view! { <HamburgerMenuIcon/> }.into_any(), "Hamburger Menu"),
                (view! { <HandIcon/> }.into_any(), "Hand"),
                (view! { <HeadingIcon/> }.into_any(), "Heading"),
                (view! { <HeartIcon/> }.into_any(), "Heart"),
                (view! { <HeartFilledIcon/> }.into_any(), "Heart Filled"),
                (view! { <HeightIcon/> }.into_any(), "Height"),
                (view! { <HobbyKnifeIcon/> }.into_any(), "Hobby Knife"),
                (view! { <HomeIcon/> }.into_any(), "Home"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsI() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <IconjarLogoIcon/> }.into_any(), "Iconjar Logo"),
                (view! { <IdCardIcon/> }.into_any(), "Id Card"),
                (view! { <ImageIcon/> }.into_any(), "Image"),
                (view! { <InfoCircledIcon/> }.into_any(), "Info Circled"),
                (view! { <InputIcon/> }.into_any(), "Input"),
                (view! { <InstagramLogoIcon/> }.into_any(), "Instagram Logo"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsK() -> impl IntoView {
    view! {
        <For
            each=move || [(view! { <KeyboardIcon/> }.into_any(), "Keyboard")]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsL() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <LapTimerIcon/> }.into_any(), "Lap Timer"),
                (view! { <LaptopIcon/> }.into_any(), "Laptop"),
                (view! { <LayersIcon/> }.into_any(), "Layers"),
                (view! { <LayoutIcon/> }.into_any(), "Layout"),
                (view! { <LetterCaseCapitalizeIcon/> }.into_any(), "Letter Case Capitalize"),
                (view! { <LetterCaseLowercaseIcon/> }.into_any(), "Letter Case Lowercase"),
                (view! { <LetterCaseToggleIcon/> }.into_any(), "Letter Case Toggle"),
                (view! { <LetterCaseUppercaseIcon/> }.into_any(), "Letter Case Uppercase"),
                (view! { <LetterSpacingIcon/> }.into_any(), "Letter Spacing"),
                (view! { <LightningBoltIcon/> }.into_any(), "Lightning Bolt"),
                (view! { <LineHeightIcon/> }.into_any(), "Line Height"),
                (view! { <Link1Icon/> }.into_any(), "Link 1"),
                (view! { <Link2Icon/> }.into_any(), "Link 2"),
                (view! { <LinkBreak1Icon/> }.into_any(), "Link Break 1"),
                (view! { <LinkBreak2Icon/> }.into_any(), "Link Break 2"),
                (view! { <LinkNone1Icon/> }.into_any(), "Link None 1"),
                (view! { <LinkNone2Icon/> }.into_any(), "Link None 2"),
                (view! { <LinkedinLogoIcon/> }.into_any(), "Linkedin Logo"),
                (view! { <ListBulletIcon/> }.into_any(), "List Bullet"),
                (view! { <LockClosedIcon/> }.into_any(), "Lock Closed"),
                (view! { <LockOpen1Icon/> }.into_any(), "Lock Open 1"),
                (view! { <LockOpen2Icon/> }.into_any(), "Lock Open 2"),
                (view! { <LoopIcon/> }.into_any(), "Loop"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsM() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <MagicWandIcon/> }.into_any(), "Magic Wand"),
                (view! { <MagnifyingGlassIcon/> }.into_any(), "Magnifying Glass"),
                (view! { <MarginIcon/> }.into_any(), "Margin"),
                (view! { <MaskOffIcon/> }.into_any(), "Mask Off"),
                (view! { <MaskOnIcon/> }.into_any(), "Mask On"),
                (view! { <MinusIcon/> }.into_any(), "Minus"),
                (view! { <MinusCircledIcon/> }.into_any(), "Minus Circled"),
                (view! { <MixIcon/> }.into_any(), "Mix"),
                (view! { <MixerHorizontalIcon/> }.into_any(), "Mixer Horizontal"),
                (view! { <MixerVerticalIcon/> }.into_any(), "Mixer Vertical"),
                (view! { <MobileIcon/> }.into_any(), "Mobile"),
                (view! { <ModulzLogoIcon/> }.into_any(), "Modulz Logo"),
                (view! { <MoonIcon/> }.into_any(), "Moon"),
                (view! { <MoveIcon/> }.into_any(), "Move"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsN() -> impl IntoView {
    view! {
        <For
            each=move || [(view! { <NotionLogoIcon/> }.into_any(), "Notion Logo")]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsO() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <OpacityIcon/> }.into_any(), "Opacity"),
                (view! { <OpenInNewWindowIcon/> }.into_any(), "Open In New Window"),
                (view! { <OverlineIcon/> }.into_any(), "Overline"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsP() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <PaddingIcon/> }.into_any(), "Padding"),
                (view! { <PaperPlaneIcon/> }.into_any(), "Paper Plane"),
                (view! { <PauseIcon/> }.into_any(), "Pause"),
                (view! { <Pencil1Icon/> }.into_any(), "Pencil 1"),
                (view! { <Pencil2Icon/> }.into_any(), "Pencil 2"),
                (view! { <PersonIcon/> }.into_any(), "Person"),
                (view! { <PieChartIcon/> }.into_any(), "Pie Chart"),
                (view! { <PilcrowIcon/> }.into_any(), "Pilcrow"),
                (view! { <PinBottomIcon/> }.into_any(), "Pin Bottom"),
                (view! { <PinLeftIcon/> }.into_any(), "Pin Left"),
                (view! { <PinRightIcon/> }.into_any(), "Pin Right"),
                (view! { <PinTopIcon/> }.into_any(), "Pin Top"),
                (view! { <PlayIcon/> }.into_any(), "Play"),
                (view! { <PlusIcon/> }.into_any(), "Plus"),
                (view! { <PlusCircledIcon/> }.into_any(), "Plus Circled"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsQ() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <QuestionMarkIcon/> }.into_any(), "Question Mark"),
                (view! { <QuestionMarkCircledIcon/> }.into_any(), "Question Mark Circled"),
                (view! { <QuoteIcon/> }.into_any(), "Quote"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsR() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <RadiobuttonIcon/> }.into_any(), "Radiobutton"),
                (view! { <ReaderIcon/> }.into_any(), "Reader"),
                (view! { <ReloadIcon/> }.into_any(), "Reload"),
                (view! { <ResetIcon/> }.into_any(), "Reset"),
                (view! { <ResumeIcon/> }.into_any(), "Resume"),
                (view! { <RocketIcon/> }.into_any(), "Rocket"),
                (view! { <RotateCounterClockwiseIcon/> }.into_any(), "Rotate Counter Clockwise"),
                (view! { <RowSpacingIcon/> }.into_any(), "Row Spacing"),
                (view! { <RowsIcon/> }.into_any(), "Rows"),
                (view! { <RulerHorizontalIcon/> }.into_any(), "Ruler Horizontal"),
                (view! { <RulerSquareIcon/> }.into_any(), "Ruler Square"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsS() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <ScissorsIcon/> }.into_any(), "Scissors"),
                (view! { <SectionIcon/> }.into_any(), "Section"),
                (view! { <SewingPinIcon/> }.into_any(), "Sewing Pin"),
                (view! { <SewingPinFilledIcon/> }.into_any(), "Sewing Pin Filled"),
                (view! { <ShadowIcon/> }.into_any(), "Shadow"),
                (view! { <ShadowInnerIcon/> }.into_any(), "Shadow Inner"),
                (view! { <ShadowNoneIcon/> }.into_any(), "Shadow None"),
                (view! { <ShadowOuterIcon/> }.into_any(), "Shadow Outer"),
                (view! { <Share1Icon/> }.into_any(), "Share 1"),
                (view! { <Share2Icon/> }.into_any(), "Share 2"),
                (view! { <ShuffleIcon/> }.into_any(), "Shuffle"),
                (view! { <SizeIcon/> }.into_any(), "Size"),
                (view! { <SketchLogoIcon/> }.into_any(), "Sketch Logo"),
                (view! { <SlashIcon/> }.into_any(), "Slash"),
                (view! { <SliderIcon/> }.into_any(), "Slider"),
                (
                    view! { <SpaceBetweenHorizontallyIcon/> }.into_any(),
                    "Space Between Horizontally",
                ),
                (view! { <SpaceBetweenVerticallyIcon/> }.into_any(), "Space Between Vertically"),
                (view! { <SpaceEvenlyHorizontallyIcon/> }.into_any(), "Space Evenly Horizontally"),
                (view! { <SpaceEvenlyVerticallyIcon/> }.into_any(), "Space Evenly Vertically"),
                (view! { <SpeakerLoudIcon/> }.into_any(), "Speaker Loud"),
                (view! { <SpeakerModerateIcon/> }.into_any(), "Speaker Moderate"),
                (view! { <SpeakerOffIcon/> }.into_any(), "Speaker Off"),
                (view! { <SpeakerQuietIcon/> }.into_any(), "Speaker Quiet"),
                (view! { <SquareIcon/> }.into_any(), "Square"),
                (view! { <StackIcon/> }.into_any(), "Stack"),
                (view! { <StarIcon/> }.into_any(), "Star"),
                (view! { <StarFilledIcon/> }.into_any(), "Star Filled"),
                (view! { <StitchesLogoIcon/> }.into_any(), "Stitches Logo"),
                (view! { <StopIcon/> }.into_any(), "Stop"),
                (view! { <StopwatchIcon/> }.into_any(), "Stopwatch"),
                (view! { <StretchHorizontallyIcon/> }.into_any(), "Stretch Horizontally"),
                (view! { <StretchVerticallyIcon/> }.into_any(), "Stretch Vertically"),
                (view! { <StrikethroughIcon/> }.into_any(), "Strikethrough"),
                (view! { <SunIcon/> }.into_any(), "Sun"),
                (view! { <SwitchIcon/> }.into_any(), "Switch"),
                (view! { <SymbolIcon/> }.into_any(), "Symbol"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsT() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <TableIcon/> }.into_any(), "Table"),
                (view! { <TargetIcon/> }.into_any(), "Target"),
                (view! { <TextIcon/> }.into_any(), "Text"),
                (view! { <TextAlignBottomIcon/> }.into_any(), "Text Align Bottom"),
                (view! { <TextAlignCenterIcon/> }.into_any(), "Text Align Center"),
                (view! { <TextAlignJustifyIcon/> }.into_any(), "Text Align Justify"),
                (view! { <TextAlignLeftIcon/> }.into_any(), "Text Align Left"),
                (view! { <TextAlignMiddleIcon/> }.into_any(), "Text Align Middle"),
                (view! { <TextAlignRightIcon/> }.into_any(), "Text Align Right"),
                (view! { <TextAlignTopIcon/> }.into_any(), "Text Align Top"),
                (view! { <TextNoneIcon/> }.into_any(), "Text None"),
                (view! { <ThickArrowDownIcon/> }.into_any(), "Thick Arrow Down"),
                (view! { <ThickArrowLeftIcon/> }.into_any(), "Thick Arrow Left"),
                (view! { <ThickArrowRightIcon/> }.into_any(), "Thick Arrow Right"),
                (view! { <ThickArrowUpIcon/> }.into_any(), "Thick Arrow Up"),
                (view! { <TimerIcon/> }.into_any(), "Timer"),
                (view! { <TokensIcon/> }.into_any(), "Tokens"),
                (view! { <TrackNextIcon/> }.into_any(), "Track Next"),
                (view! { <TrackPreviousIcon/> }.into_any(), "Track Previous"),
                (view! { <TransformIcon/> }.into_any(), "Transform"),
                (view! { <TransparencyGridIcon/> }.into_any(), "Transparency Grid"),
                (view! { <TrashIcon/> }.into_any(), "Trash"),
                (view! { <TriangleDownIcon/> }.into_any(), "Triangle Down"),
                (view! { <TriangleLeftIcon/> }.into_any(), "Triangle Left"),
                (view! { <TriangleRightIcon/> }.into_any(), "Triangle Right"),
                (view! { <TriangleUpIcon/> }.into_any(), "Triangle Up"),
                (view! { <TwitterLogoIcon/> }.into_any(), "Twitter Logo"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsU() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <UnderlineIcon/> }.into_any(), "Underline"),
                (view! { <UpdateIcon/> }.into_any(), "Update"),
                (view! { <UploadIcon/> }.into_any(), "Upload"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsV() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <ValueIcon/> }.into_any(), "Value"),
                (view! { <ValueNoneIcon/> }.into_any(), "Value None"),
                (view! { <VercelLogoIcon/> }.into_any(), "Vercel Logo"),
                (view! { <VideoIcon/> }.into_any(), "Video"),
                (view! { <ViewGridIcon/> }.into_any(), "View Grid"),
                (view! { <ViewHorizontalIcon/> }.into_any(), "View Horizontal"),
                (view! { <ViewNoneIcon/> }.into_any(), "View None"),
                (view! { <ViewVerticalIcon/> }.into_any(), "View Vertical"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsW() -> impl IntoView {
    view! {
        <For
            each=move || [(view! { <WidthIcon/> }.into_any(), "Width")]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsZ() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <ZoomInIcon/> }.into_any(), "Zoom In"),
                (view! { <ZoomOutIcon/> }.into_any(), "Zoom Out"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
