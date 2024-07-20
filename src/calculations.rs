use crate::server;
use plotters::prelude::*;
use serde::Serialize;

/// Represents an investment with principal, contribution, interest rate, and duration.
#[derive(Debug)]
pub struct Investment {
    /// The initial amount of money invested.
    pub principal: f64,
    /// The monthly contribution added to the investment.
    pub contribution: f64,
    /// The annual interest rate as a percentage.
    pub rate: f64,
    /// The number of years the money is invested for.
    pub years: i32,
}

impl Investment {
    /// Creates an `Investment` instance from command line arguments.
    ///
    /// # Arguments
    ///
    /// * `matches` - The command line argument matches containing investment parameters.
    ///
    /// # Returns
    ///
    /// Returns an `Investment` instance with values parsed from command line arguments.
    ///
    /// # Example
    ///
    /// ```
    /// use cic::calculations::Investment;
    ///
    /// let matches = clap::Command::new("investment")
    ///     .arg(clap::Arg::new("principal").default_value("0"))
    ///     .arg(clap::Arg::new("contribution").default_value("1"))
    ///     .arg(clap::Arg::new("rate").default_value("5"))
    ///     .arg(clap::Arg::new("years").default_value("5"))
    ///     .get_matches();
    /// let investment = Investment::from_matches(&matches);
    /// ```
    pub fn from_matches(matches: &clap::ArgMatches) -> Self {
        Self {
            principal: matches
                .get_one::<String>("principal")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0),
            contribution: matches
                .get_one::<String>("contribution")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1.0),
            rate: matches
                .get_one::<String>("rate")
                .and_then(|s| s.parse().ok())
                .unwrap_or(5.0),
            years: matches
                .get_one::<String>("years")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        }
    }

    /// Creates an `Investment` instance from the provided `InvestmentParams`.
    ///
    /// # Arguments
    ///
    /// * `params` - An instance of `InvestmentParams` containing the parameters for the investment.
    ///   This includes the principal amount, monthly contribution, annual interest rate, and duration in years.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self, &'static str>`. On success, returns an `Investment` instance initialized with
    /// the provided parameters. If any of the values are negative, returns an error with a message indicating
    /// that negative values are not allowed.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions are met:
    /// - `params.principal` is less than 0.0
    /// - `params.contribution` is less than 0.0
    /// - `params.rate` is less than 0.0
    /// - `params.years` is less than 0
    ///
    /// # Example
    ///
    /// ```
    /// use cic::server::InvestmentParams;
    /// use cic::calculations::Investment;
    ///
    /// let params = InvestmentParams {
    ///     principal: 1000.0,
    ///     contribution: 100.0,
    ///     rate: 5.0,
    ///     years: 10,
    /// };
    ///
    /// match Investment::from_params(params) {
    ///     Ok(investment) => println!("Investment created: {:?}", investment),
    ///     Err(e) => eprintln!("Error creating investment: {}", e),
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic but returns an error if invalid values are provided.
    pub fn from_params(params: server::InvestmentParams) -> Result<Self, &'static str> {
        if params.principal < 0.0
            || params.contribution < 0.0
            || params.rate < 0.0
            || params.years < 0
        {
            return Err("Negative values are not allowed");
        }
        Ok(Self {
            principal: params.principal,
            contribution: params.contribution,
            rate: params.rate,
            years: params.years,
        })
    }

    /// Generates a yearly summary of the investment.
    ///
    /// # Returns
    ///
    /// Returns a vector of `YearlySummary` structs, each representing the investment's status at the end of each year.
    ///
    /// # Example
    ///
    /// ```
    /// use cic::calculations::YearlySummary;
    /// use cic::calculations::Investment;
    ///
    /// let investment = Investment {
    ///     principal: 1000.0,
    ///     contribution: 100.0,
    ///     rate: 5.0,
    ///     years: 10,
    /// };
    /// let summary = investment.yearly_summary();
    /// ```
    pub fn yearly_summary(&self) -> Vec<YearlySummary> {
        let rate_per_period = self.rate / 100.0;
        let mut amount = self.principal;
        let mut total_interest = 0.0;
        let mut summary = Vec::with_capacity(self.years as usize);

        for year in 1..=self.years {
            let annual_contribution = self.contribution * 12.0;
            let annual_interest = amount * rate_per_period;
            total_interest += annual_interest;

            amount += annual_contribution + annual_interest;

            summary.push(YearlySummary {
                year,
                principal: self.principal,
                annual_contribution,
                total_contribution: self.contribution * 12.0 * year as f64,
                annual_interest,
                total_interest,
                total_amount: amount,
            });
        }
        summary
    }
}

/// Represents a summary of the investment at the end of a given year.
#[derive(Debug, Serialize)]
pub struct YearlySummary {
    /// The year for which the summary is provided.
    pub year: i32,
    /// The initial principal amount of the investment.
    pub principal: f64,
    /// The total contribution made during the year.
    pub annual_contribution: f64,
    /// The cumulative total contribution up to the end of the year.
    pub total_contribution: f64,
    /// The interest earned during the year.
    pub annual_interest: f64,
    /// The cumulative total interest earned up to the end of the year.
    pub total_interest: f64,
    /// The total amount of money at the end of the year.
    pub total_amount: f64,
}

/// Plots the investment summary as a line chart.
///
/// # Arguments
///
/// * `summary` - A slice of `YearlySummary` structs representing the investment's progress over time.
///
/// # Returns
///
/// Returns `Result<(), Box<dyn std::error::Error>>` indicating success or failure of the plotting process.
///
/// # Example
///
/// ```no_run
/// use cic::calculations::plot_summary;
/// use cic::calculations::YearlySummary;
///
/// let summary = vec![
///     YearlySummary { year: 1, principal: 1000.0, annual_contribution: 1200.0, total_contribution: 1200.0, annual_interest: 50.0, total_interest: 50.0, total_amount: 2150.0 },
///     // Add more summaries here
/// ];
/// plot_summary(&summary).expect("Failed to plot summary");
/// ```
pub fn plot_summary(summary: &[YearlySummary]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Investment Summary", ("sans-serif", 30).into_font())
        .x_label_area_size(35)
        .y_label_area_size(100)
        .margin(20)
        .build_cartesian_2d(
            1..summary.len(),
            0.0..summary
                .iter()
                .map(|s| s.total_amount)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Year")
        .y_desc("Amount")
        .draw()?;

    let years: Vec<usize> = summary.iter().map(|s| s.year as usize).collect();
    let mut principal_and_contribution: Vec<f64> = Vec::new();
    let mut accumulated_principal_and_contribution = 0.0;
    for s in summary {
        accumulated_principal_and_contribution += s.annual_contribution;
        principal_and_contribution.push(s.principal + accumulated_principal_and_contribution);
    }
    let total_amount: Vec<f64> = summary.iter().map(|s| s.total_amount).collect();

    chart
        .draw_series(LineSeries::new(
            years
                .iter()
                .zip(principal_and_contribution.iter())
                .map(|(x, y)| (*x, *y)),
            &RED,
        ))?
        .label("Principal + Contribution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            years.iter().zip(total_amount.iter()).map(|(x, y)| (*x, *y)),
            &BLUE,
        ))?
        .label("Total Amount")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .draw()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_investment_initialization() {
        let investment = Investment {
            principal: 1000.0,
            contribution: 100.0,
            rate: 5.0,
            years: 10,
        };

        assert_eq!(investment.principal, 1000.0);
        assert_eq!(investment.contribution, 100.0);
        assert_eq!(investment.rate, 5.0);
        assert_eq!(investment.years, 10);
    }

    #[test]
    fn test_yearly_summary() {
        let investment = Investment {
            principal: 1000.0,
            contribution: 100.0,
            rate: 5.0,
            years: 3,
        };

        let summary = investment.yearly_summary();
        assert_eq!(summary.len(), 3);

        // Year 1
        assert_eq!(summary[0].year, 1);
        assert_eq!(summary[0].principal, 1000.0);
        assert_eq!(summary[0].annual_contribution, 1200.0);
        assert!((summary[0].annual_interest - 50.0).abs() < 1e-2);
        assert_eq!(summary[0].total_contribution, 1200.0);
        assert!((summary[0].total_interest - 50.0).abs() < 1e-2);
        assert!((summary[0].total_amount - 2250.0).abs() < 1e-2);

        // Year 2
        assert_eq!(summary[1].year, 2);
        assert_eq!(summary[1].principal, 1000.0);
        assert_eq!(summary[1].annual_contribution, 1200.0);
        assert!((summary[1].annual_interest - 112.5).abs() < 1e-2);
        assert_eq!(summary[1].total_contribution, 2400.0);
        assert!((summary[1].total_interest - 162.5).abs() < 1e-2);
        assert!((summary[1].total_amount - 3562.5).abs() < 1e-2);

        // Year 3
        assert_eq!(summary[2].year, 3);
        assert_eq!(summary[2].principal, 1000.0);
        assert_eq!(summary[2].annual_contribution, 1200.0);
        assert!((summary[2].annual_interest - 178.125).abs() < 1e-2);
        assert_eq!(summary[2].total_contribution, 3600.0);
        assert!((summary[2].total_interest - 340.625).abs() < 1e-2);
        assert!((summary[2].total_amount - 4940.625).abs() < 1e-2);
    }
}
