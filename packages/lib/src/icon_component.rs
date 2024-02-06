use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape: Clone {
    fn view_box(&self) -> String;
    fn xmlns(&self) -> String;
    fn child_elements(&self) -> Element;
}
#[derive(Clone, Props, PartialEq)]
pub struct IconProps<T: IconShape + 'static + PartialEq> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `<svg>` element. Defaults to 20.
    #[props(default = 20.to_string())]
    pub height: String,
    /// The width of the `<svg>` element. Defaults to 20.
    #[props(default = 20.to_string())]
    pub width: String,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// An class for the `<svg>` element.
    #[props(default)]
    pub class: String,
    /// An accessible, short-text description for the icon.
    #[props(default)]
    pub title: String,
}

impl<T: IconShape + Clone + PartialEq> IconProps<T> {
    pub fn new(
        class: impl Into<String>,
        height: impl Into<String>,
        width: impl Into<String>,
        fill: impl Into<String>,
        title: impl Into<String>,
        icon: T,
    ) -> Self {
        IconProps {
            class: class.into(),
            height: height.into(),
            width: width.into(),
            fill: fill.into(),
            title: title.into(),
            icon,
        }
    }
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<'a, T: IconShape + Clone + PartialEq>(props: IconProps<T>) -> Element {
    rsx! {
        svg {
            stroke: "currentColor",
            stroke_width: "0",
            class: format_args!("{}", props.class),
            height: format_args!("{}", props.height),
            width: format_args!("{}", props.width),
            view_box: format_args!("{}", props.icon.view_box()),
            xmlns: format_args!("{}", props.icon.xmlns()),
            fill: format_args!("{}", props.fill),
            title {
                "{props.title}"
            }
            {
                props.icon.child_elements()
            }
        }
    }
}
