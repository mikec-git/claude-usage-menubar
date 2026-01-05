use crate::parser::types::TokenUsage;

pub struct ModelPricing {
    pub input_per_million: f64,
    pub output_per_million: f64,
    pub cache_creation_per_million: f64,
    pub cache_read_per_million: f64,
}

pub fn get_pricing(model: &str) -> ModelPricing {
    let model_lower = model.to_lowercase();

    if model_lower.contains("opus-4-5") || model_lower.contains("opus-4.5") {
        ModelPricing {
            input_per_million: 5.00,
            output_per_million: 25.00,
            cache_creation_per_million: 6.25,
            cache_read_per_million: 0.50,
        }
    } else if model_lower.contains("sonnet-4-5")
        || model_lower.contains("sonnet-4.5")
        || model_lower.contains("3-5-sonnet")
        || model_lower.contains("3.5-sonnet")
    {
        ModelPricing {
            input_per_million: 3.00,
            output_per_million: 15.00,
            cache_creation_per_million: 3.75,
            cache_read_per_million: 0.30,
        }
    } else if model_lower.contains("sonnet-4-") || model_lower.contains("sonnet-4.") {
        ModelPricing {
            input_per_million: 3.00,
            output_per_million: 15.00,
            cache_creation_per_million: 3.75,
            cache_read_per_million: 0.30,
        }
    } else if model_lower.contains("haiku-4-5")
        || model_lower.contains("haiku-4.5")
        || model_lower.contains("3-5-haiku")
        || model_lower.contains("3.5-haiku")
    {
        ModelPricing {
            input_per_million: 1.00,
            output_per_million: 5.00,
            cache_creation_per_million: 1.25,
            cache_read_per_million: 0.10,
        }
    } else if model_lower.contains("haiku") {
        ModelPricing {
            input_per_million: 0.25,
            output_per_million: 1.25,
            cache_creation_per_million: 0.30,
            cache_read_per_million: 0.03,
        }
    } else if model_lower.contains("opus") {
        ModelPricing {
            input_per_million: 15.00,
            output_per_million: 75.00,
            cache_creation_per_million: 18.75,
            cache_read_per_million: 1.50,
        }
    } else if model_lower.contains("sonnet") {
        ModelPricing {
            input_per_million: 3.00,
            output_per_million: 15.00,
            cache_creation_per_million: 3.75,
            cache_read_per_million: 0.30,
        }
    } else {
        ModelPricing {
            input_per_million: 3.00,
            output_per_million: 15.00,
            cache_creation_per_million: 3.75,
            cache_read_per_million: 0.30,
        }
    }
}

pub fn calculate_cost(model: &str, usage: &TokenUsage) -> f64 {
    let pricing = get_pricing(model);

    let input_cost = (usage.input_tokens as f64 / 1_000_000.0) * pricing.input_per_million;
    let output_cost = (usage.output_tokens as f64 / 1_000_000.0) * pricing.output_per_million;
    let cache_creation_cost = (usage.cache_creation_input_tokens.unwrap_or(0) as f64 / 1_000_000.0)
        * pricing.cache_creation_per_million;
    let cache_read_cost = (usage.cache_read_input_tokens.unwrap_or(0) as f64 / 1_000_000.0)
        * pricing.cache_read_per_million;

    input_cost + output_cost + cache_creation_cost + cache_read_cost
}
