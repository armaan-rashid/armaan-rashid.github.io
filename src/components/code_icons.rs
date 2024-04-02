use leptos::*;
use web_sys::MouseEvent;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub enum LanguageIcon {
    Rust,
    Python,
    Haskell,
    Swift,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct LanguageIconData {
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: u32,
    pub height: u32,
    /// These are string literals for Tailwind detection.
    pub hover_height: &'static str,
    pub hover_color: &'static str,
}

static TRUNC_HEIGHT: u32 = 175;
static DEFAULT_FILL: &'static str = "#D9D9D9";

// Modularly packaging the &self visual data.
impl LanguageIcon {
    pub fn bg_color(&self) -> &'static str {
        match &self {
            LanguageIcon::Rust => "bg-rustdark",
            LanguageIcon::Python => "bg-pythondark",
            LanguageIcon::Haskell => "bg-haskelldark",
            LanguageIcon::Swift => "bg-swiftdark",
        }
    }

    pub fn component(&self) -> impl IntoView {
        match &self {
            LanguageIcon::Rust => view! {<RustIcon/>},
            LanguageIcon::Python => view! {<PythonIcon/>},
            LanguageIcon::Haskell => view! {<HaskellIcon/>},
            LanguageIcon::Swift => view! {<SwiftIcon/>},
        }
    }

    pub fn x_offset(&self) -> i32 {
        match &self {
            LanguageIcon::Rust => -50,
            LanguageIcon::Python => -50,
            LanguageIcon::Haskell => 0,
            LanguageIcon::Swift => -50,
        }
    }

    pub fn y_offset(&self) -> i32 {
        match &self {
            LanguageIcon::Rust => 73,
            LanguageIcon::Python => 73,
            LanguageIcon::Haskell => 60,
            LanguageIcon::Swift => 80,
        }
    }

    pub fn width(&self) -> u32 {
        match &self {
            LanguageIcon::Rust => 382,
            LanguageIcon::Python => 372,
            LanguageIcon::Haskell => 404,
            LanguageIcon::Swift => 362,
        }
    }

    pub fn hover_height(&self) -> &'static str {
        match &self {
            LanguageIcon::Rust => "hover:h-[362px]",
            LanguageIcon::Python => "hover:h-[362px]",
            LanguageIcon::Haskell => "hover:h-[404px]",
            LanguageIcon::Swift => "hover:h-[404px]",
        }
    }

    pub fn height(&self) -> u32 {
        match &self {
            LanguageIcon::Rust => 362,
            LanguageIcon::Python => 362,
            LanguageIcon::Haskell => 404,
            LanguageIcon::Swift => 404,
        }
    }

    pub fn hover_color(&self) -> &'static str {
        match &self {
            LanguageIcon::Rust => "hover:fill-rust",
            LanguageIcon::Python => "hover:fill-python",
            LanguageIcon::Haskell => "hover:fill-haskell",
            LanguageIcon::Swift => "hover:fill-swift",
        }
    }

    pub fn fill_color(&self) -> &'static str {
        match &self {
            LanguageIcon::Rust => "fill-rust",
            LanguageIcon::Python => "fill-python",
            LanguageIcon::Haskell => "fill-haskell",
            LanguageIcon::Swift => "fill-swift",
        }
    }

    pub fn icon_data(&self) -> LanguageIconData {
        LanguageIconData {
            x_offset: self.x_offset(),
            y_offset: self.y_offset(),
            width: self.width(),
            height: self.height(),
            hover_height: self.hover_height(),
            hover_color: self.hover_color(),
        }
    }
}

// SVG constants. Nicer to have them here than hiding in files, there's not so many.

#[component]
pub fn GitHubIcon(fill: &'static str) -> impl IntoView {
    view! {
        <svg width="512" height="500" viewBox="0 0 512 500" fill={fill} xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M236 1.12405C220.64 2.15305 200.348 5.79505 184.398 10.3851C94.215 36.3431 24.071 112.149 5.431 203.8C-8.168 270.664 4.559 339.283 40.824 394.631C66.33 433.557 98.758 463.24 137.615 483.23C151.747 490.499 171.702 498.609 177.896 499.599C183.689 500.526 188.788 497.735 191.179 492.328C192.956 488.311 193.04 486.326 192.396 463.725L191.703 439.439L183.101 440.771C173.44 442.267 157.467 441.73 144.528 439.474C134.048 437.647 123.394 432.352 116.685 425.636C111.378 420.323 110.009 418.016 103.057 402.674C96.777 388.813 89.61 379.613 79.926 372.981C73.251 368.41 67.771 362.425 68.204 360.178C68.78 357.186 73.033 355.858 80.319 356.394C94.388 357.428 109.118 367.703 119.612 383.803C126.798 394.827 133.886 401.678 142.557 405.98C148.798 409.077 150.511 409.463 159.5 409.798C169.833 410.183 180.182 408.623 188.582 405.416C192.731 403.831 192.866 403.633 194.389 396.89C196.326 388.31 199.984 380.465 205.014 374.101L208.879 369.212L200.689 368.06C174.317 364.35 155.537 357.914 138.023 346.584C115.098 331.753 101.487 308.664 94.94 273.5C92.903 262.556 92.583 234.594 94.377 224.305C97.284 207.639 104.157 192.356 114.403 179.778L118.879 174.284L116.939 168.051C111.711 151.253 112.704 130.941 119.74 110.75C121.507 105.679 121.939 105.474 130 105.89C141.78 106.497 167.074 116.651 184.279 127.679L191.443 132.271L196.971 131.188C200.012 130.593 205.742 129.405 209.706 128.549C236.802 122.694 275.198 122.694 302.294 128.549C306.258 129.405 311.975 130.577 315 131.153L320.5 132.2L330 126.423C352.638 112.658 378.44 103.418 387.484 105.837C390.054 106.525 390.776 107.548 392.751 113.303C397.053 125.845 398.259 134.308 397.754 148.431C397.414 157.943 396.72 163.24 395.204 167.892L393.121 174.284L397.495 179.653C406.383 190.564 413.032 204.056 416.706 218.637C418.937 227.494 419.183 260.318 417.112 273C413.879 292.802 405.673 313.585 396.12 326.167C379.6 347.925 349.676 362.769 311.803 367.992L303.105 369.192L306.977 374.091C311.744 380.12 315.47 387.848 317.626 396.176C318.989 401.439 319.328 409.884 319.647 446.5C320.073 495.461 320.061 495.378 326.757 498.555C331.379 500.748 335.124 500.076 349.726 494.436C423.474 465.952 481.738 401.198 502.999 324.091C517.131 272.843 514.142 213.336 494.957 163.987C477.842 119.96 447.458 79.7651 410.162 51.8101C369.719 21.4961 323.661 4.39105 273.517 1.06105C256.002 -0.101947 254.267 -0.0989469 236 1.12405Z"/>
        </svg>

    }
}

#[component]
pub fn HaskellIcon() -> impl IntoView {
    view! {
        <path d="M1.59194 1.91526L1.59185 1.9151L0.834633 0.5H35.298H70.3296L117.58 71.2716L164.793 141.988L117.581 212.716L70.3316 283.5H35.299H0.834658L1.64384 281.988L1.64391 281.988C1.7135 281.858 2.09987 281.249 2.8011 280.17C3.4928 279.105 4.47351 277.609 5.71133 275.73C8.18683 271.971 11.6889 266.68 15.9601 260.247C24.5022 247.379 36.1194 229.939 48.7498 211.034L94.6778 142.286L94.8633 142.008L94.6778 141.73L48.6978 72.8783C36.0535 53.9441 24.4293 36.4893 15.8871 23.6201C11.616 17.1855 8.11558 11.8976 5.64356 8.14467C4.40748 6.26808 3.42893 4.77587 2.73983 3.71614C2.04096 2.6414 1.65794 2.03872 1.59194 1.91526ZM189.819 142.283L190.004 142.006L189.819 141.729L144.329 73.4757C119.307 35.9343 98.034 3.98713 97.0499 2.47701L95.7624 0.50013L130.205 0.505001L165.303 0.509961L259.919 142.283L354.165 283.5H319.586H284.34L255.237 239.751C247.193 227.658 239.805 216.661 234.383 208.684C231.673 204.696 229.452 201.461 227.887 199.222C227.104 198.103 226.483 197.23 226.046 196.635C225.828 196.338 225.651 196.106 225.522 195.945C225.458 195.866 225.399 195.797 225.349 195.744C225.325 195.718 225.294 195.686 225.258 195.657C225.251 195.651 225.242 195.644 225.232 195.637C225.216 195.625 225.197 195.611 225.174 195.598C225.148 195.583 225.06 195.534 224.936 195.529L224.935 195.528C224.801 195.523 224.703 195.572 224.673 195.587C224.631 195.609 224.599 195.633 224.58 195.648C224.541 195.678 224.508 195.711 224.483 195.736C224.432 195.789 224.372 195.859 224.308 195.937C224.178 196.096 224.001 196.327 223.782 196.622C223.343 197.214 222.72 198.084 221.935 199.201C220.364 201.434 218.137 204.663 215.418 208.648C209.979 216.617 202.568 227.612 194.498 239.71L165.302 283.474L130.205 283.487L95.7611 283.5L97.0453 281.522C98.0252 280.014 119.299 248.069 144.323 210.53L189.819 142.283ZM276.811 129.5L261.271 106.222L246.101 83.5H324.118H402.57V106.5V129.5H339.807H276.811ZM310.487 180.172C303.059 168.911 296.161 158.365 295.154 156.729L293.781 154.5H347.978H402.57V177.5V200.499L363.321 200.457L323.839 200.413L310.487 180.172Z"/>
    }
}

#[component]
pub fn SwiftIcon() -> impl IntoView {
    view! {
        <path d="M285.987 219.497C285.978 219.483 285.969 219.473 285.958 219.46C286.415 217.721 286.887 215.993 287.287 214.211C304.44 137.837 262.577 47.5425 191.736 0C222.78 47.0324 236.506 103.999 224.311 153.818C223.223 158.262 221.916 162.526 220.47 166.671C218.902 165.52 216.926 164.212 214.272 162.577C214.272 162.577 143.804 113.952 67.4284 27.9462C65.4244 25.6886 108.155 96.2018 156.649 153.46C133.802 139.13 70.129 87.3559 29.8199 46.1248C34.7722 55.355 40.6626 64.2429 47.1375 72.7995C80.7991 120.508 124.698 179.374 177.291 224.576C140.339 249.848 88.1243 251.813 36.1368 224.602C23.2782 217.867 11.1899 209.742 0 200.509C22.0062 239.847 55.8989 273.788 97.1495 293.602C146.342 317.228 195.261 315.626 231.696 293.989L231.667 294.036C231.834 293.918 232.045 293.79 232.218 293.671C233.714 292.772 235.199 291.857 236.651 290.89C254.157 280.735 288.733 270.433 307.292 310.789C311.837 320.665 321.496 268.325 285.987 219.497Z"/>
    }
}

#[component]
pub fn PythonIcon() -> impl IntoView {
    view! {
        <path fill-rule="evenodd" clip-rule="evenodd" d="M88.5458 18.4959C96.309 8.26246 113.514 2.5572 143.007 0.437163C161.305 -0.878029 189.692 0.894461 202.415 4.14494C211.675 6.51192 217.561 9.88953 224.574 16.8606C230.669 22.918 233.838 28.2995 235.465 35.3556C235.988 37.6229 236.38 56.4773 236.393 80.0163C236.417 123.438 236.168 126.375 231.745 135.045C229.029 140.37 221.716 147.822 216.332 150.752C207.638 155.484 203.257 155.858 156.326 155.876C119.634 155.889 112.543 156.107 108.309 157.35C93.0832 161.823 81.4531 172.618 75.1461 188.136C73.3919 192.452 73.1486 195.227 72.7206 215.798L72.245 238.662L59.4406 238.922C33.3261 239.452 23.271 235.285 13.2406 219.778C4.39643 206.105 -1.70577 172.742 0.427074 149.718C1.66818 136.322 5.27078 119.918 8.80205 111.586C14.093 99.1003 24.6877 89.2016 37.4902 84.7841C41.5739 83.3747 48.3237 83.1552 100.598 82.7364L159.132 82.2662V77.236V72.2057L120.887 71.967L82.6412 71.7282L82.9951 50.4739C83.3802 27.3209 83.8338 24.7088 88.5458 18.4959ZM130.322 37.6768C130.322 32.9045 127.724 28.4861 123.076 25.3545C116.944 21.2214 108.434 23.1677 103.725 29.7793C100.797 33.8932 100.643 42.0825 103.425 45.8415C112.556 58.1776 130.322 52.7851 130.322 37.6768ZM231.077 302.784C223.314 313.017 206.109 318.723 176.617 320.843C158.318 322.158 129.931 320.385 117.208 317.135C107.949 314.768 102.062 311.39 95.049 304.419C88.9542 298.362 85.7851 292.98 84.158 285.924C83.6348 283.657 83.2434 264.802 83.2297 241.263C83.2059 197.842 83.4547 194.904 87.8777 186.235C90.594 180.91 97.9072 173.458 103.291 170.528C111.985 165.795 116.366 165.421 163.297 165.404C199.989 165.391 207.08 165.173 211.314 163.93C226.54 159.457 238.17 148.661 244.477 133.144C246.231 128.828 246.474 126.052 246.902 105.482L247.378 82.6173L260.182 82.3576C286.297 81.828 296.352 85.9949 306.382 101.502C315.227 115.175 321.329 148.538 319.196 171.562C317.955 184.958 314.352 201.361 310.821 209.694C305.53 222.179 294.935 232.078 282.133 236.496C278.049 237.905 271.299 238.125 219.026 238.543L160.491 239.014V244.044V249.074L198.736 249.313L236.982 249.552L236.628 270.806C236.243 293.959 235.789 296.571 231.077 302.784ZM189.301 283.603C189.301 288.375 191.899 292.794 196.547 295.925C202.679 300.058 211.189 298.112 215.898 291.5C218.826 287.387 218.98 279.197 216.198 275.438C207.067 263.102 189.301 268.495 189.301 283.603Z"/>
    }
}

#[component]
pub fn RustIcon() -> impl IntoView {
    view! {
        <path fill-rule="evenodd" clip-rule="evenodd" d="M161.8 1.59652C161.028 2.36852 158.633 5.96052 156.479 9.57852C153.723 14.2075 151.961 16.1615 150.533 16.1745C149.347 16.1845 145.817 13.4765 142.033 9.65352C133.054 0.58152 131.689 0.836519 127.11 12.4385C124.881 18.0845 123 21.3935 121.803 21.7735C120.561 22.1675 117.516 20.6805 112.592 17.2765C105.142 12.1265 102.1 11.1755 100.331 13.4415C99.7941 14.1295 98.7731 18.2915 98.0621 22.6915C96.1301 34.6425 95.3941 35.0515 84.5621 30.1915C72.5191 24.7885 70.4211 26.3375 71.5431 39.8075C72.2251 47.9905 72.1911 48.1755 69.7941 49.2675C67.9131 50.1245 65.9191 50.0145 61.1261 48.7865C47.5761 45.3155 46.0531 46.5635 48.5901 59.0615C49.5781 63.9255 50.1021 68.6445 49.7551 69.5485C49.2301 70.9185 47.4901 71.1915 39.2931 71.1915C25.4531 71.1915 24.8591 72.1065 30.5651 84.6275C35.1151 94.6135 34.6881 95.4775 24.3571 97.1695C15.6701 98.5925 12.0621 100.329 12.0621 103.085C12.0621 104.101 14.3121 108.019 17.0621 111.791C24.2141 121.601 23.9951 122.391 13.0621 126.237C0.883081 130.522 0.350081 132.834 9.56208 141.415C18.1561 149.421 18.1171 150.721 9.11708 156.159C5.29708 158.467 1.66208 160.969 1.04008 161.719C-1.45692 164.728 0.412081 167.532 8.06208 172.255C18.0861 178.444 18.2151 179.675 9.64908 187.406C-0.178919 196.277 0.469081 198.471 14.3541 203.32C23.4671 206.503 23.7421 207.874 17.1491 217.262C9.79108 227.739 10.3071 229.523 21.2011 231.277C31.3051 232.903 31.9481 233.111 33.0151 235.103C33.7971 236.565 33.2391 238.692 30.5431 244.526C24.7031 257.166 25.6881 258.719 39.0631 257.956C51.3201 257.257 51.2461 257.132 47.9891 272.974C46.8341 278.589 46.8741 279.433 48.3651 280.924C49.9221 282.481 50.6611 282.468 59.5001 280.73C72.0451 278.264 72.3831 278.521 71.4631 289.865C70.8751 297.107 71.0251 298.493 72.5781 300.21C74.9961 302.882 75.8911 302.763 85.1121 298.548C92.5491 295.148 93.2071 295.013 94.8921 296.538C96.0591 297.594 97.1621 301.029 98.0091 306.239C99.2341 313.785 100.849 317.192 103.199 317.192C104.722 317.192 108.038 315.255 113.856 310.966C121.51 305.325 122.72 305.836 126.831 316.443C131.495 328.477 132.893 328.79 141.929 319.824C145.646 316.136 149.489 313.193 150.587 313.194C151.945 313.196 153.811 315.245 156.562 319.759C163.969 331.911 166.168 332.036 173.265 320.702C176.638 315.316 178.549 313.192 180.024 313.192C181.26 313.192 184.591 315.844 188.384 319.847C193.857 325.623 195.032 326.416 197.28 325.852C199.358 325.331 200.457 323.765 202.827 317.947C204.452 313.957 206.056 309.904 206.391 308.942C207.41 306.01 210.74 306.95 218.537 312.37C225.118 316.945 226.266 317.42 228.395 316.45C230.387 315.542 231.011 314.166 231.992 308.522C233.887 297.618 233.99 297.301 235.973 296.239C237.435 295.457 239.562 296.015 245.396 298.711C257.876 304.477 259.28 303.618 258.989 290.388C258.881 285.465 259.131 280.89 259.544 280.221C260.36 278.9 263.59 279.132 273.759 281.24C283.708 283.303 284.452 281.717 280.933 265.947C279.178 258.081 279.779 257.615 291.157 258.027C304.486 258.511 305.508 256.809 299.631 243.913C295.229 234.254 295.707 233.452 306.941 231.653C319.965 229.568 320.759 227.68 313.062 217.097C306.007 207.397 306.118 207.035 317.483 202.758C321.29 201.325 325.002 199.656 325.733 199.05C328.452 196.794 327.083 193.655 320.562 187.192C316.987 183.649 314.062 180.011 314.062 179.108C314.062 178.146 317.378 175.355 322.062 172.374C328.282 168.415 330.059 166.771 330.047 164.987C330.025 161.733 329.421 161.123 321.312 156.165C316.034 152.938 314.062 151.163 314.062 149.64C314.062 148.318 316.477 145.414 320.617 141.756C326.371 136.673 327.135 135.585 326.867 132.851C326.581 129.927 326.044 129.543 318.107 126.588C306.697 122.341 306.337 121.343 312.937 112.269C315.756 108.394 318.062 104.385 318.062 103.362C318.062 101.065 315.026 98.1915 312.599 98.1915C311.605 98.1915 307.996 97.6765 304.577 97.0465C295.914 95.4505 295.466 94.3345 299.661 84.8225C305.207 72.2475 304.574 71.1915 291.484 71.1915C279.436 71.1915 279.283 71.0065 281.65 59.2975C284.218 46.5905 283.076 45.5935 269.541 48.7325C259.31 51.1055 259.062 50.8825 259.062 39.3245C259.062 25.5945 257.674 24.7475 244.783 30.6225C235.162 35.0065 234.417 34.5585 232.447 23.2065C230.186 10.1775 228.689 9.56452 217.736 17.1785C208.44 23.6415 207.417 23.3645 203.687 13.3845C202.106 9.15352 200.363 5.12952 199.815 4.44152C197.699 1.78952 194.459 3.23752 188.062 9.69152C184.519 13.2665 180.835 16.1915 179.876 16.1915C178.791 16.1915 176.195 13.1695 173.001 8.19152C167.655 -0.14248 165.065 -1.66848 161.8 1.59652ZM170.985 31.7805C175.165 35.2975 175.367 41.6135 171.426 45.5555C165.565 51.4155 155.062 47.0095 155.062 38.6915C155.062 35.9865 155.821 34.2785 157.985 32.1145C161.816 28.2835 166.681 28.1585 170.985 31.7805ZM151.562 54.1915C159.769 62.4375 160.845 63.1915 164.398 63.1915C167.825 63.1915 169.105 62.3985 175.419 56.3665C185.697 46.5445 187.298 45.1915 188.639 45.1915C192.153 45.1915 208.839 50.9185 217.562 55.1185C235.285 63.6535 250.054 75.6965 261.504 90.9525C271.322 104.035 271.099 101.786 264.131 117.536C260.793 125.081 258.062 132.543 258.062 134.118C258.062 138.246 261.176 140.718 273.086 146.042C278.848 148.618 284.125 151.082 284.812 151.518C285.675 152.065 286.062 155.387 286.062 162.251V172.192H279.207H272.352L271.768 179.163C270.982 188.537 268.92 192.598 263.653 195.148C258.54 197.623 253.064 197.761 248.37 195.534C244.476 193.686 243.817 192.506 241.512 183.252C239.226 174.07 236.589 169.483 229.878 163.016L223.987 157.339L228.275 154.384C234.126 150.351 242.345 141.479 245.194 136.119C247.164 132.414 247.547 130.224 247.546 122.692C247.544 114.729 247.214 113.064 244.683 108.238C241.021 101.259 233.141 93.5755 225.429 89.4665C213.775 83.2575 213.305 83.2225 141.229 83.2065C89.4171 83.1945 75.9801 82.9325 76.3021 81.9415C76.9991 79.8005 93.2051 66.2955 100.761 61.5595C109.864 55.8545 123.38 49.8925 133.062 47.3115C137.187 46.2125 141.021 45.2855 141.583 45.2525C142.144 45.2185 146.635 49.2415 151.562 54.1915ZM51.3781 118.309C54.9021 120.111 57.0161 123.438 57.0401 127.219C57.0671 131.548 55.1021 134.518 51.1211 136.167C45.0091 138.699 38.0621 134.18 38.0621 127.672C38.0621 123.101 39.3891 120.622 42.8371 118.751C46.1481 116.954 48.4921 116.833 51.3781 118.309ZM286.467 119.331C287.841 119.957 289.692 121.999 290.579 123.869C295.189 133.583 282.657 141.941 274.985 134.269C267.134 126.417 276.182 114.645 286.467 119.331ZM83.0621 157.692V196.192H66.1211C56.8031 196.192 48.9691 195.851 48.7111 195.434C46.2301 191.418 43.3811 164.799 44.4861 155.952L45.1761 150.429L55.8691 145.731C61.7501 143.147 67.5751 140.091 68.8121 138.938C71.6391 136.306 71.7331 131.888 69.0841 126.051C66.0081 119.273 66.0791 119.192 75.0621 119.192H83.0621V157.692ZM183.29 121.378C192.87 124.249 196.387 133.216 190.031 138.565C185.294 142.55 181.038 143.18 158.812 143.186L138.062 143.192V131.692V120.192H158.696C171.721 120.192 180.79 120.629 183.29 121.378ZM177.864 177.993C184.279 181.412 187.352 187.034 189.961 200.129C191.167 206.186 192.799 213.31 193.586 215.962C195.634 222.856 200.603 229.839 205.508 232.713C209.719 235.181 209.854 235.192 235.957 235.192C265.656 235.192 263.648 234.349 255.528 243.412L251.494 247.914L239.667 245.443C222.664 241.891 221.669 242.602 217.649 261.191C216.16 268.077 214.546 273.096 213.6 273.788C211.274 275.489 197.655 279.804 189.024 281.575C166.648 286.167 142.844 284.045 120.812 275.495C117.099 274.055 114.062 272.729 114.062 272.55C114.062 272.371 112.917 266.704 111.518 259.958C107.816 242.105 106.408 241.225 88.0151 245.272L78.5621 247.352L73.2311 241.522L67.9011 235.692L118.231 235.192L168.562 234.692V216.692V198.692L153.312 198.416L138.062 198.14V186.585V175.03L155.812 175.365C172.037 175.672 173.932 175.898 177.864 177.993ZM98.9861 258.661C106.124 265.8 97.5771 278.133 88.1871 274.244C85.0931 272.962 82.0621 268.483 82.0621 265.192C82.0621 262.268 84.9041 257.599 87.4821 256.289C90.6321 254.688 96.1501 255.826 98.9861 258.661ZM244.139 259.115C246.303 261.279 247.062 262.987 247.062 265.692C247.062 270.726 242.639 275.192 237.655 275.192C232.4 275.192 227.862 270.848 227.862 265.817C227.862 257.077 237.948 252.924 244.139 259.115Z"/>
    }
}
#[component]
pub fn CodeIcon(
    lang: LanguageIcon,
    #[prop(default = "#D9D9D9")] fill: &'static str,
    #[prop(optional)] stroke: &'static str,
) -> impl IntoView {
    let LanguageIconData { width, height, .. } = lang.icon_data();
    let s = "60%";
    view! {
            <svg //width=s
                 //height=s
                 viewBox={format!("0 0 {} {}", width, height)}
                 fill={fill}
                 stroke={stroke}
                 xmlns="http://www.w3.org/2000/svg">
                 {lang.component()}
            </svg>
    }
}
#[component]
pub fn CodeIconPop(
    lang: LanguageIcon,
    #[prop(default = 1.0)] scale: f32,
    #[prop(default = true)] offset: bool,
    #[prop(default = DEFAULT_FILL)] fill: &'static str,
    #[prop(optional)] stroke: &'static str,
    #[prop(default = None)] init_height: Option<u32>,
) -> impl IntoView {
    let LanguageIconData {
        x_offset,
        y_offset,
        width,
        height,
        hover_height,
        hover_color,
    } = lang.icon_data();
    let h = scale * init_height.unwrap_or(height) as f32;
    let w = scale * width as f32;
    view! {
        <svg width=w
             height={h}
             viewBox={move || format!("{} {} {} {}",if offset {x_offset} else {0},
                                                    if offset {y_offset} else {0},
                                                    w,h)}
             fill={fill}
             stroke={stroke}
             class=format!("transition-all hover:scale-105 hover:drop-shadow-lg
                            hover:backdrop-blue-3xl {} {}", hover_height, hover_color)
             xmlns="http://www.w3.org/2000/svg">
             {lang.component()}
        </svg>
    }
}

#[component]
pub fn LanguageStack(lang_name: WriteSignal<&'static str>) -> impl IntoView {
    let rust_name = move |_: MouseEvent| lang_name("Rust");
    let python_name = move |_: MouseEvent| lang_name("Python");
    let haskell_name = move |_: MouseEvent| lang_name("Haskell");
    let swift_name = move |_: MouseEvent| lang_name("Swift");
    let no_name = move |_: MouseEvent| lang_name("");
    view! {
        <div class="flex flex-col gap-12 l-12">
            <a href="/code/rust"><CodeIconPop on:mouseover=rust_name
                      on:mouseleave=no_name
                      init_height=Some(TRUNC_HEIGHT)
            lang=LanguageIcon::Rust/></a>
            <a href="/code/python"><CodeIconPop on:mouseover=python_name
                      on:mouseleave=no_name
                      init_height=Some(TRUNC_HEIGHT)
                      lang=LanguageIcon::Python/></a>
            <a href="/code/haskell"><CodeIconPop on:mouseover=haskell_name
                      on:mouseleave=no_name
                      init_height=Some(TRUNC_HEIGHT)
                      lang=LanguageIcon::Haskell/></a>
            <a href="/code/swift"><CodeIconPop on:mouseover=swift_name
                      on:mouseleave=no_name
                      init_height=Some(TRUNC_HEIGHT)
                      lang=LanguageIcon::Swift/></a>
        </div>
    }
}