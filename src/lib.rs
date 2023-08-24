use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn generate_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let item_clone = item.clone();
  let input = parse_macro_input!(item_clone as DeriveInput);
  let name = &input.ident;

  let write_to_json = quote! {
    {
      let _FAKED_STRUCT_: #name = Faker.fake();
      let json_response = serde_json::to_string_pretty(&_FAKED_STRUCT_).unwrap();

      // Get the directory to write this to from env.
      let path = std::env::var("generated_models_directory");

      // If the directory is not set, then just use "/generated_models"
      let path = match path {
        Ok(path) => path,
        Err(_) => "/generated_models".to_string(),
      };

      // Create the directory if it doesn't exist.
      let folder_path = format!("{}/{}", std::env::current_dir().unwrap().display(), path);
      std::fs::create_dir_all(&folder_path).unwrap();

      // Write the file to the directory.
      let file_path = format!("{}/{}.json", folder_path, stringify!(#name));
      std::fs::write(file_path, json_response).unwrap();
    }
  };

  let input = quote! {
    #input
  };

  let output = quote! {
    #input
    #write_to_json
  };

  TokenStream::from(output)
}
