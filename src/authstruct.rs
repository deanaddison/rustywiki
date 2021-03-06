use std::{
    collections::HashMap,
    error,
    ops::Deref,
    sync::RwLock
};

use rocket::http::{Cookies, Cookie};

use super::{
    user::{User, AuthState},
    wikifile
};

#[derive(Serialize, Deserialize, Debug)]
/// Wrapper for auth data
pub struct AuthStruct (HashMap<String, UserStruct>);
impl Deref for AuthStruct {
    type Target = HashMap<String, UserStruct>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Use to load and write to file
#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename = "Userlist")]
    pub user_list: Vec<UserStruct>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserStruct {
	user : String, 
	password : String, 
	salt : String, 
	comment : String 
}

/// Tries to load the user file for a tiny wiki
pub fn load_auth() -> Result<wikifile::WikiStruct<AuthStruct>, Box<dyn error::Error>> {
	return Ok(wikifile::WikiStruct(RwLock::new( load_auth_int()? )))
}

/// Tries to load the user file to an wiki container
pub fn load_auth_int() -> Result<wikifile::WikiContainer<AuthStruct>, Box<dyn error::Error>> {
    // TODO, look at cleaning this code up a bit
    if let Ok((um, hdr)) = wikifile::load_parts("site/wiki/_user/current") {
        let umwin: Wrapper = serde_json::from_str(&um)?;
        let umap = AuthStruct(umwin.user_list.iter().map(|us| (us.user.clone(), us.clone())).collect());
        return Ok(wikifile::WikiContainer{data: umap, header: hdr})
    }
    Err("Failed to load".into())
}


// TODO - not happy with the encapsulation, look at refactoring
pub fn login_handle(uname: &str, pwd: &str, cookies: &mut Cookies<'_>, umap: &wikifile::WikiStruct<AuthStruct>) -> Option<User> {
    let thing = &umap.read().unwrap().data;
    // TODO handle no auth case
    let entry = thing.get(uname)?;
    if entry.password != pwd { return None };
    let u_tok = User {auth:AuthState::AuthAdmin, name: uname.to_string()}; // TODO get auth from list of admin
    cookies.add_private(Cookie::new("wiki_auth", u_tok.to_string()));
    Some(u_tok)
}

