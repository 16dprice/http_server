pub enum Route {
    Root,
    Sleep,
}

pub struct RouteDoesNotExistError;

impl Route {
    pub fn from_string(route_str: String) -> Result<Route, RouteDoesNotExistError> {
        return match &route_str[..] {
            "/" => Ok(Route::Root),
            "/sleep" => Ok(Route::Sleep),
            _ => Err(RouteDoesNotExistError)
        }
    }

    pub fn to_string(&self) -> String {
        return match &self {
            Route::Root => "/".to_string(),
            Route::Sleep => "/sleep".to_string()
        }
    }
}
