# Population Scam Simulation

During a discussion on Reddit, the following was posited:

"Not true, the young fall for scams much more than seniors. Please do not perpetuate this stereotype."

It is important to acknowledge first and foremost that ageism and age-based discrimination are real and wrong.  
The purpose of this project is not to demonstrate that seniors fall for scams more often than the young, nor is the purpose of this article to discredit any point of the author (but one).  Our goal is to demonstrate the power of confounding variables and the impacts of population size.

### Context:

The [New York Times article](https://www.nytimes.com/2021/06/25/your-money/young-seniors-scams-warning.html) claims the following:

> "For years now, the Better Business Bureau’s survey research has shown that younger adults lose money to swindlers much more often than the older people you may think of as the stereotypical victims. ... 
> The Federal Trade Commission reports similar figures, with 44 percent of people ages 20 to 29 losing money to fraud, more than double the 20 percent of people ages 70 to 79."

If one dives deeper into the [FTC data](https://www.ftc.gov/system/files/ftc_gov/pdf/CSN%20Annual%20Data%20Book%202021%20Final%20PDF.pdf), the following appears prominently on page six of the report:

> "Of people who reported their age, those aged 20-29 reported losing money to fraud in 41% of reports filed with the FTC, while people aged 70 – 79 reported losing money in 18% of their reports and people 80 and over reported it in 17% of their reports. But when they did experience a loss, people aged 70 and older reported much higher median losses than any other age group."
>
> "Younger people reported losing money to fraud more often than older people."

Does this mean we can conclude that persons aged 20-29 are twice as likely to fall for scams as individuals aged 70-79?

No.  

We will offer some alternative conclusions and highlight some confounding variables which are equally capable of explaining the numbers.
Additionally, this repository provides a tool wherein the reader can control and play with the hidden variables.  
Give it a try and pay attention to how different root causes OTHER than (or in addition to) age contribute to the frequency of outcomes.

#### Alternative explanation: Individuals aged 20-29 are a larger part of the population, and thus will report scams more often.

The 2019 census data says that 13.7% of the population is aged between 20 and 29 years of age.  (44.5M people)
7.2% of the population is aged between 70 and 79 years of age. (23.4M people)

Page 13 says that 189,639 of 1.3 million reports were submitted by individuals aged 20-29.
164,902 of the 1.3 million reports were submitted by individuals aged 70-79.

If we assume one person submits only one report (unknown!) then 0.8% of individuals aged 20 to 29 years have reported a scam while 0.7% of individuals aged 70-79 have reported a scam.

#### Alternative explanation: Fraud reports by younger individuals are diluted by non-scam content.

The FTC report enumerates a few things that are considered fraud:

- Misleading pitches for donations to benefit a charity; solicitations for bogus charity or relief organizations; etc.
- difficulty canceling an ISP or online account
- issues with online payment services, social networking services, internet gaming, and virtual reality
- undisclosed charges
- problems with broadband internet services and content, including the truthfulness of cost, access, and speed disclosures

It is not clear if these are adding to the monetary values in the P13 numbers.  

#### Alternative explanation: Individuals aged between 20-29 are more likely to shop online.

Purchasing an item and never receiving it is counted as a scam, per [the BBB report](https://bbbfoundation.images.worldnow.com/library/65016b74-abf5-456b-9604-892e46ebc7dd.pdf).
If a person is more likely to be buying a product online, they're more likely to fall prey to an online scam.

#### Alternative explanation: Individuals aged 20-29 are more likely to be victims of scams involving identity theft by relatives, inflating the net number of reports without indicating that the individuals themselves are more easily scammed.

Parents, guardians, and custodians have access to their charges' social security numbers, bank accounts, and personal information.  If the scam reports include numbers for this group, that could conceivably inflate the scam rate for individuals.

# Development:

### Building Webapp:

1. cargo build -p webapp

### Developing Webapp:

1. trunk  

```toml
# netlify.toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```
