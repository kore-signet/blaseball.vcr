use blaseball_vcr::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Serialize;
use serde_json::Value as JSONValue;

#[macro_export]
macro_rules! etypes {
    ($e:ident, $f:ident, $m: ident, $($name:literal > $what:ty),*) => {
        match $e.as_str() {
            $(
                $name => $f::<$what>($e, $m).await,
            )*
            _ => panic!()
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "page")]
    pub next_page: Option<String>,
    #[serde(rename = "type")]
    pub entity_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    pub count: u32,
    pub before: String,
}

pub async fn v2_paged_get(
    client: &reqwest::Client,
    url: &str,
    mpb: &MultiProgress,
    mut parameters: ChroniclerParameters,
) -> anyhow::Result<Vec<ChroniclerEntity<JSONValue>>> {
    let mut results: Vec<ChroniclerEntity<JSONValue>> = Vec::new();

    let mut page = 1;
    let spinny = mpb.add(ProgressBar::new_spinner());
    spinny.enable_steady_tick(std::time::Duration::from_millis(120));
    spinny.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    loop {
        spinny.set_message(format!("downloading entities - page {}", page));
        let mut chron_response: ChroniclerResponse<ChroniclerEntity<JSONValue>> = client
            .get(url)
            .query(&parameters)
            .send()
            .await?
            .json()
            .await?;
        results.append(&mut chron_response.items);

        if let Some(next_page) = chron_response.next_page {
            parameters.next_page = Some(next_page);
            page += 1;
        } else {
            break;
        }
    }
    mpb.remove(&spinny);

    Ok(results)
}
