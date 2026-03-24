use axum::{Json, Router, extract::State, routing::get};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;

use crate::{
    models::skill::{Aura, Hissatsu}, routes::common::Language, state::SharedState
};



pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/", get(get_hissatsu))
        .route("/aura", get(get_auras))
}

async fn get_hissatsu(
    State(app_state): State<SharedState>,
    Query(params) : Query<SkillSearchParams>
) -> Result<Json<Vec<Hissatsu>>, axum::http::StatusCode> {
    let mut query_builder = QueryBuilder::new(
&format!("
SELECT skill_id, n.name, power, element, category, growth_rate, is_block, is_longshot, tp_consumption, cooldown
FROM skills.hissatsu
JOIN {}.skill_names n
ON n.id = skills.hissatsu.name_id
", params.language.to_sql())
    );

    let mut where_used = false;

    if !params.element.is_empty() {
        query_builder.push("WHERE element IN (");
        where_used = true;

        let mut separated = query_builder.separated(",");
        for element in &params.element {
            separated.push_bind(*element as u8);
        }
        separated.push_unseparated(")\n");
    }

    if !params.category.is_empty() {
        if !where_used {
            query_builder.push("WHERE category IN (");
            where_used = true;
        } else {
            query_builder.push("AND category IN (");
        }

        let mut separated = query_builder.separated(",");
        for category in &params.category {
            separated.push_bind(*category as u8);
        }
        separated.push_unseparated(")\n");
    }

    if params.is_block {
        if !where_used {
            query_builder.push("WHERE is_block=true\n");
            where_used = true;
        } else {
            query_builder.push("AND is_block=true\n");
        }
    }

    if params.is_longshot {
        if !where_used {
            query_builder.push("WHERE is_longshot=true\n");
        } else {
            query_builder.push("AND is_longshot=true\n");
        }
    }

    query_builder.push("ORDER BY power");
    if params.descending { query_builder.push(" DESC"); }

    #[cfg(debug_assertions)]
    println!("{}", query_builder.sql());

    let query = query_builder.build();
    let result = query
        .fetch_all(app_state.pool())
        .await;

    match result {
        Ok(skills) => {
            let skills = skills.into_iter()
                .filter_map(Hissatsu::parse)
                .collect();

            Ok(Json(skills))
        }

        Err(e) => {
            eprintln!("{e}");
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_auras(
    State(app_state): State<SharedState>,
    Query(params) : Query<SkillSearchParams>
) -> Result<Json<Vec<Aura>>, axum::http::StatusCode> {
    let mut query_builder = QueryBuilder::new(
&format!("
SELECT 
    aura_id, 
    n1.name AS aura_name, 
    aura_type, 
    a.element AS aura_element, 
    ah.skill_id, 
    n2.name, 
    ah.power, 
    ah.element, 
    ah.category, 
    ah.growth_rate, 
    ah.is_block, 
    ah.is_longshot, 
    ah.tp_consumption, 
    ah.cooldown

FROM skills.aura a

JOIN {0}.skill_names n1
    ON n1.id = a.name_id

LEFT JOIN skills.aura_hissatsu ah
    ON ah.skill_id = a.skill_id

LEFT JOIN {0}.skill_names n2
    ON n2.id = ah.name_id
", params.language.to_sql())
    );

    let mut where_used = false;

    if !params.element.is_empty() {
        query_builder.push("WHERE aura_element IN (");
        where_used = true;

        let mut separated = query_builder.separated(",");
        for element in &params.element {
            separated.push_bind(*element as u8);
        }
        separated.push_unseparated(")\n");
    }

    if !params.category.is_empty() {
        if !where_used {
            query_builder.push("WHERE category IN (");
        } else {
            query_builder.push("AND category IN (");
        }

        let mut separated = query_builder.separated(",");
        for category in &params.category {
            separated.push_bind(*category as u8);
        }
        separated.push_unseparated(")\n");
    }

    if !params.aura_type.is_empty() {
        if !where_used {
            query_builder.push("WHERE aura_type IN (");
        } else {
            query_builder.push("AND aura_type IN (");
        }

        let mut separated = query_builder.separated(",");
        for aura_type in &params.aura_type {
            separated.push_bind(*aura_type as u8);
        }
        separated.push_unseparated(")\n");
    }
    

    let query = query_builder.build();
    let result = query
        .fetch_all(app_state.pool())
        .await;

    match result {
        Ok(skills) => {
            let skills = skills.into_iter()
                .filter_map(Aura::parse)
                .collect();

            Ok(Json(skills))
        }

        Err(e) => {
            eprintln!("{e}");
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash)]
struct SkillSearchParams {
    #[serde(default)]
    name: String,

    #[serde(default)]
    element: Vec<crate::models::common::Element>,

    #[serde(default)]
    category: Vec<crate::models::skill::Category>,

    #[serde(default)]
    aura_type: Vec<crate::models::skill::AuraType>,

    #[serde(default)]
    is_block: bool,

    #[serde(default)]
    is_longshot: bool,

    #[serde(default)]
    descending: bool,

    #[serde(default = "Language::default")]
    language: Language
}