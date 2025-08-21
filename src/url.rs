use crate::utils::API_HOST;

#[inline(always)]
pub fn market_time() -> String {
    format!("{}/market-time", *API_HOST)
}

#[inline(always)]
pub fn user_info() -> String {
    format!("{}/user-info", *API_HOST)
}

// #[inline(always)]
// pub fn account_list() -> String {
//     format!("{}/account-list", *API_HOST)
// }

#[inline(always)]
pub fn position() -> String {
    format!("{}/position", *API_HOST)
}

#[inline(always)]
pub fn account_balances() -> String {
    format!("{}/balances", *API_HOST)
}

#[inline(always)]
pub fn account_history() -> String {
    format!("{}/history", *API_HOST)
}

#[inline(always)]
pub fn watchlists() -> String {
    format!("{}/watchlists", *API_HOST)
}

#[inline(always)]
pub fn watchlist_quote(id: u8) -> String {
    format!("{}/watchlists/{}", *API_HOST, id)
}

#[inline(always)]
pub fn watchlist() -> String {
    format!("{}/watchlist", *API_HOST)
}

#[inline(always)]
pub fn single_quote(symbol: &str) -> String {
    format!("{}/stock-quote/{}", *API_HOST, symbol)
}

// #[inline(always)]
// pub fn stock_mohlc(symbols: &str, resolution: u8) -> String {
//     format!("{}/stock-ohlc/{}?resolution={}", *API_HOST, symbols, resolution)
// }

#[inline(always)]
pub fn fundamental(symbol: &str) -> String {
    format!("{}/fundamental/{}", *API_HOST, symbol)
}

#[inline(always)]
pub fn company_profile(symbol: &str) -> String {
    format!("{}/company-profile/{}", *API_HOST, symbol)
}

#[inline(always)]
pub fn cash_dividend(symbol: &str) -> String {
    format!("{}/cash-dividend/{}", *API_HOST, symbol)
}

#[inline(always)]
pub fn corp_calendar(symbol: &str) -> String {
    format!("{}/corp-calendar/{}", *API_HOST, symbol)
}
