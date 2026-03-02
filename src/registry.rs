use meme_generator_core::{meme::Meme, registry::MemeRegistry};

pub(crate) struct MemeDeclaration {
    pub name: &'static str,
    pub builder: fn() -> Box<dyn Meme>,
}

inventory::collect!(MemeDeclaration);

#[macro_export]
macro_rules! register_meme {
    ($key:expr, $function:expr, $($field:ident = $value:expr),* $(,)?) => {
        inventory::submit! {
            $crate::registry::MemeDeclaration {
                name: $key,
                builder: || -> Box<dyn meme_generator_core::meme::Meme> {
                    Box::new(
                        meme_generator_utils::builder::MemeBuilder {
                            key: $key.to_string(),
                            function: $function,
                            $(
                                $field: meme_generator_utils::builder::meme_setters::$field($value),
                            )*
                            ..Default::default()
                        }
                    )
                }
            }
        }
    }
}

#[allow(improper_ctypes_definitions)]
extern "C" fn register_memes(registry: &mut dyn MemeRegistry) {
    for meme_declaration in inventory::iter::<MemeDeclaration> {
        registry.register_meme(meme_declaration.name, (meme_declaration.builder)());
    }
}

#[unsafe(no_mangle)]
pub static MEME_PACK_DECLARATION: meme_generator_core::registry::MemePackDeclaration =
    meme_generator_core::registry::MemePackDeclaration {
        rustc_version: meme_generator_core::registry::RUSTC_VERSION,
        core_version: meme_generator_core::registry::CORE_VERSION,
        register: register_memes,
    };