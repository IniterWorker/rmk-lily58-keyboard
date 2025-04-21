use rmk::action::KeyAction;
use rmk::{a, k};
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 10;
pub(crate) const NUM_LAYER: usize = 4;

// TODO: I am using vial.
// See: vil-files
#[rustfmt::skip]
#[allow(dead_code)] // some kind of issue with the clippy linter and the paths.
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5)],
            [a!(No), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            [a!(No),k!(A), k!(S), k!(D), k!(F), k!(G)],
            [a!(No),k!(Z), k!(X), k!(C), k!(V), k!(B)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            
            [k!(Enter), k!(Kc0), k!(Kc9),  k!(Kc8),  k!(Kc7), k!(Kc6)],
            [a!(No), k!(P), k!(O),k!(I), k!(U), k!(Y)],
            [a!(No), a!(No), k!(L), k!(J), k!(K), k!(H)],
            [a!(No), a!(No), a!(No), a!(No), k!(M),  k!(N)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ],
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5)],
            [a!(No), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            [a!(No),k!(A), k!(S), k!(D), k!(F), k!(G)],
            [a!(No),k!(Z), k!(X), k!(C), k!(V), k!(B)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            
            [k!(Enter), k!(Kc0), k!(Kc9),  k!(Kc8),  k!(Kc7), k!(Kc6)],
            [a!(No), k!(P), k!(O),k!(I), k!(U), k!(Y)],
            [a!(No), a!(No), k!(L), k!(J), k!(K), k!(H)],
            [a!(No), a!(No), a!(No), a!(No), k!(M),  k!(N)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]
        ,
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5)],
            [a!(No), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            [a!(No),k!(A), k!(S), k!(D), k!(F), k!(G)],
            [a!(No),k!(Z), k!(X), k!(C), k!(V), k!(B)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            
            [k!(Enter), k!(Kc0), k!(Kc9),  k!(Kc8),  k!(Kc7), k!(Kc6)],
            [a!(No), k!(P), k!(O),k!(I), k!(U), k!(Y)],
            [a!(No), a!(No), k!(L), k!(J), k!(K), k!(H)],
            [a!(No), a!(No), a!(No), a!(No), k!(M),  k!(N)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ],
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5)],
            [a!(No), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            [a!(No),k!(A), k!(S), k!(D), k!(F), k!(G)],
            [a!(No),k!(Z), k!(X), k!(C), k!(V), k!(B)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), k!(No)],
            
            [k!(Enter), k!(Kc0), k!(Kc9),  k!(Kc8),  k!(Kc7), k!(Kc6)],
            [a!(No), k!(P), k!(O),k!(I), k!(U), k!(Y)],
            [a!(No), a!(No), k!(L), k!(J), k!(K), k!(H)],
            [a!(No), a!(No), a!(No), a!(No), k!(M),  k!(N)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]
    ]
}
