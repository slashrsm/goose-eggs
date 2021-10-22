use goose::prelude::*;

use crate::common;

use rand::seq::SliceRandom;

/// Load the front page in English and all static assets found on the page.
pub async fn front_page_en(user: &mut GooseUser) -> GooseTaskResult {
    let goose = user.get("").await?;
    goose_eggs::validate_and_load_static_assets(user, goose, &goose_eggs::Validate::title("Home"))
        .await?;

    Ok(())
}

/// Load recipe listing in English and all static assets found on the page.
pub async fn recipe_listing_en(user: &mut GooseUser) -> GooseTaskResult {
    let goose = user.get("en/recipes/").await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title("Recipes"),
    )
    .await?;

    Ok(())
}

/// Load a random recipe in English and all static assets found on the page.
pub async fn recipe_en(user: &mut GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(recipe.unwrap().url_en).await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title(recipe.unwrap().title_en),
    )
    .await?;

    Ok(())
}

/// Load article listing in English and all static assets found on the page.
pub async fn article_listing_en(user: &mut GooseUser) -> GooseTaskResult {
    let goose = user.get("en/articles/").await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title("Articles"),
    )
    .await?;

    Ok(())
}

/// Load a random article in English and all static assets found on the page.
pub async fn article_en(user: &mut GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(article.unwrap().url_en).await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title(article.unwrap().title_en),
    )
    .await?;

    Ok(())
}

/// Load a random basic page in English and all static assets found on the page.
pub async fn basic_page_en(user: &mut GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(page.unwrap().url_en).await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title(page.unwrap().title_en),
    )
    .await?;

    Ok(())
}

/// Load a random node by nid in English and all static assets found on the page.
pub async fn page_by_nid(user: &mut GooseUser) -> GooseTaskResult {
    // Randomly select a content type.
    let content_types = vec![
        common::ContentType::Article,
        common::ContentType::BasicPage,
        common::ContentType::Recipe,
    ];
    let content_type = content_types.choose(&mut rand::thread_rng());
    // Then randomly select a node of this content type.
    let nodes = common::get_nodes(content_type.unwrap());
    let page = nodes.choose(&mut rand::thread_rng());
    // Load the page by nid instead of by URL.
    let goose = user
        .get(&("node/".to_string() + &page.unwrap().nid.to_string()))
        .await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title(page.unwrap().title_en),
    )
    .await?;

    Ok(())
}

/// Anonymously load the contact form in English and POST feedback.
pub async fn anonymous_contact_form_en(user: &mut GooseUser) -> GooseTaskResult {
    common::anonymous_contact_form(user, true).await?;

    Ok(())
}

// Pick a random word from the title of a random node and perform a search in English.
pub async fn search_en(user: &mut GooseUser) -> GooseTaskResult {
    // Build a random three-word phrase taken from english words on the Umami website.
    let search_words = common::random_words(3, true);
    let search_phrase = search_words.join(" ");

    // The search page should have "Search" in the title.
    let validate_search_page = &goose_eggs::Validate::title("Search");
    // The results page should have the search_prhase in the title.
    let validate_results_page = &goose_eggs::Validate::title(&search_phrase);
    let search_params = goose_eggs::drupal::SearchParams::builder()
        .keys(&*search_phrase)
        .url("en/search/node")
        .search_page_validation(validate_search_page)
        .results_page_validation(validate_results_page)
        .build();
    goose_eggs::drupal::search(user, &search_params).await?;

    Ok(())
}

/// Load category listing by a random term in English and all static assets found on the page.
pub async fn term_listing_en(user: &mut GooseUser) -> GooseTaskResult {
    let terms = common::get_terms();
    let term = terms.choose(&mut rand::thread_rng());
    let goose = user.get(term.unwrap().url_en).await?;
    goose_eggs::validate_and_load_static_assets(
        user,
        goose,
        &goose_eggs::Validate::title(term.unwrap().title_en),
    )
    .await?;

    Ok(())
}
