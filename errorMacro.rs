/// Macro for error processing
/// 
/// This macro just creates a new [`DSError`] object. Does not translate.
/// In order to translate, the [`DSError`] object needs to be [`filled`](DSError::apply_user) with a user's paramas, like language.
/// 
/// This macro processes a variable number of parameters in two groups separated by a semicolon (;):  
/// It returns a struct of type [`DSError`], which can be processed in message translation.
///
/// ### First group: one to three parameters
/// * The first one (Mandatory) is the translation key of the message, in the pattern "mod.sec.key", or "mod.sec.key:code" if error has a code number (string)
/// * The second one (Optional) is the lang to translate to.
/// * The third one (Optional) is a message to override the code.
///
/// ### Second group (Optional)
/// Parameters to include in the final message, separated by commas.
///
/// ## Examples
/// 
/// *Only the translation key*
/// ```text
/// let ds_error=dserror!("mod.sec.key");
/// ```
/// 
/// *Translation key and lang*
/// ```text
/// let ds_error=dserror!("mod.sec.key","es");
/// ```
/// 
/// *Only the message* (No password or language required)
/// ```text
/// let ds_error=dserror!("","","No trans message");
/// ```
/// 
/// 
/// *Translation key, lang and parameters*
/// ```text
/// let ds_error=dserror!("mod.sec.key","en";"param1","param2");
/// ```
/// 

#[macro_export]
macro_rules! dserror{
    // No params
    ($k:expr) => {
        // Key...
        DSError::from_key($k)
    };
    ($k:expr,$l:expr) => {{
        // Key & Message
        let mut ds=DSError::from_key($k);
        ds.lang=$l.to_string();
        ds
    }};
    ($k:expr,$l:expr,$m:expr) => {{
        // Key & Message
        let mut ds=DSError::from_key($k);
        ds.lang=$l.to_string();
        ds.message=$m.to_string();
        ds
    }};
    // With n params
    ($k:expr; $($p:expr),+) => {{
        // Make a Vec
        let mut params: Vec<String> = Vec::new();
        $(
            params.push($p.to_string());
        )*
        // Generate
        let mut ds=DSError::from_key($k);
        ds.params=params;
        ds
    }};
    ($k:expr,$l:expr; $($p:expr),+) => {{
        // Make a Vec
        let mut params: Vec<String> = Vec::new();
        $(
            params.push($p.to_string());
        )*
        // Generate
        let mut ds=DSError::from_key($k);
        ds.params=params;
        ds.lang=$l.to_string();
        ds
    }};
    ($k:expr,$l:expr,$m:expr; $($p:expr),+) => {{
        // Make a Vec
        let mut params: Vec<String> = Vec::new();
        $(
            params.push($p.to_string());
        )*
        // Generate
        let mut ds=DSError::from_key($k);
        ds.params=params;
        ds.lang=$l.to_string();
        ds.message=$m.to_string();
        ds
    }};
}
