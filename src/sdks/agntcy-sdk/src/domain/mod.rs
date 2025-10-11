#[cfg(feature = "domain")]
#[doc = "A comprehensive table outlining distinct fields of application and knowledge areas within the framework."]
pub enum Domain {
    #[doc = "Development, management, and use of systems, devices, and software to solve problems and enhance human capabilities."]
    Technology,
    #[doc = "Protecting systems, data, and networks from cyber threats and vulnerabilities. Subdomains: Cybersecurity, Data Security, Application Security, Network Security, and Identity Management."]
    Security,
    #[doc = "Technologies for transmitting and receiving information over various media. Subdomains: Telecommunication, Wireless Communication, Signal Processing, and Broadcasting Systems."]
    CommunicationSystems,
    #[doc = "All aspects of managing and supporting technology systems and infrastructure. Subdomains: Network Monitoring, Network Monitoring, Incident Management, Network Performance Analysis, Traffic Management, Network Configuration, Network Optimization, and Network Troubleshooting."]
    InformationTechnology,
    #[doc = "Connecting everyday objects to the internet for data exchange and automation. Subdomains: IoT Devices, IoT Security, IoT Networks, Smart Homes, and Industrial IoT."]
    InternetOfThings,
    #[doc = "Tasks like monitoring, configuring, and optimizing networks. Subdomains: Network Configuration, Network Optimization, Network Troubleshooting, Wireless Networks, Network Protocols, Network Architecture, and Network Security."]
    NetworkManagement,
    #[doc = "Ensures the smooth operation and performance of network infrastructure. Subdomains: Network Monitoring, Incident Management, Network Performance Analysis, and Traffic Management."]
    NetworkOperations,
    #[doc = "Design, management, and security of computer networks. Subdomains: Wireless Networks, Network Protocols, Network Architecture, and Network Security."]
    Networking,
    #[doc = "Designing, developing, and maintaining software applications and systems. Subdomains: Front-end Development, Back-end Development, Full-stack Development, DevOps, and Quality Assurance."]
    SoftwareEngineering,
    #[doc = "Managing money, investments, and financial risks within businesses or for individuals. Subdomains: Corporate Finance, Personal Finance, Risk Management, Accounting, and Financial Analysis."]
    FinanceAndBusiness,
    #[doc = "Operations of financial institutions involved in accepting deposits, lending money, and other financial services. Subdomains: Retail Banking, Investment Banking, Corporate Banking, and Digital Banking."]
    Banking,
    #[doc = "Creation, marketing, and distribution of products intended for personal use. Subdomains: Product Development, Consumer Behavior, Marketing, Retail, and Supply Chain Management."]
    ConsumerGoods,
    #[doc = "Managing money, investments, and financial risks within businesses or for individuals. Subdomains: Corporate Finance, Personal Finance, Risk Management, Accounting, and Financial Analysis."]
    Finance,
    #[doc = "Managing financial assets, investment portfolios, and providing advisory services to clients. Subdomains: Asset Management, Hedge Funds, Private Equity, Mutual Funds, and Financial Planning."]
    InvestmentServices,
    #[doc = "Sale of goods and services directly to consumers through various channels. Subdomains: E-commerce, In-store Retail, Inventory Management, Customer Experience, and Omnichannel Retail."]
    Retail,
    #[doc = "cientific research and innovations in biology, genetics, biotechnology, and other related fields, with the goal of understanding life processes and advancing medical and environmental solutions. Subdomains: Biotechnology, Pharmaceutical Research, Genomics, Bioinformatics, and Molecular Biology."]
    LifeScience,
    #[doc = "the application of computational techniques to analyze and interpret biological data. Subdomains: Sequence Analysis, Systems Biology, Data Mining, and Structural Bioinformatics."]
    Bioinformatics,
    #[doc = "The application of biological systems and organisms to develop or create products and technologies that improve the quality of human life. Subdomains: Medical Biotechnology, Agricultural Biotechnology, Industrial Biotechnology, and Environmental Biotechnology."]
    Biotechnology,
    #[doc = "The study of genomes to understand genetic structure, function, and evolution. Subdomains: Comparative Genomics, Functional Genomics, Population Genomics, and Metagenomics."]
    Genomics,
    #[doc = "Molecular Biology is the branch of biology that focuses on the structure, function, and interactions of biological macromolecules, such as DNA, RNA, and proteins. Subdomains: Genomics, Gene Expression, and Cell Signaling."]
    MolecularBiology,
    #[doc = "The discovery, development, and testing of new drugs and therapies to treat diseases and improve health outcomes. Subdomains: Drug Discovery, Clinical Trials, Pharmacology, and Regulatory Affairs."]
    PharmaceuticalResearch,
    #[doc = "Maintaining a secure and reliable environment, primarily online, by managing risks, preventing harm, and ensuring safety and privacy. Subdomains: Online Safety, Content Moderation, Fraud Prevention, Data Privacy, and Risk Management."]
    TrustAndSafety,
    #[doc = "Reviewing and managing user-generated content to ensure it complies with community guidelines and legal standards. Subdomains: Automated Moderation, Community Guidelines, Human Moderation, and Harmful Content Detection."]
    ContentModeration,
    #[doc = "Safeguarding personal information from unauthorized access and ensuring compliance with privacy laws and regulations. Subdomains: Privacy Regulations Compliance, Data Encryption, Data Anonymization, and User Consent Management."]
    DataPrivacy,
    #[doc = "Identifying and stopping fraudulent activities to protect individuals and organizations from financial and reputational damage. Subdomains: Transaction Monitoring, Identity Verification, Fraud Analytics, and Fraud Awareness Training."]
    FraudPrevention,
    #[doc = "Protecting internet users from various forms of harm to ensure a secure digital environment. Subdomains: Cybersecurity Awareness, Child Online Protection, Identity Protection, and Digital Wellbeing."]
    OnlineSafety,
    #[doc = "Identifying, assessing, and prioritizing risks to minimize their impact on an organization's objectives and operations. Subdomains: Risk Assessment, Mitigation Strategies, Crisis Management, and Compliance and Auditing."]
    RiskManagement,
    #[doc = "Managing and optimizing the workforce of an organization, focusing on recruitment, employee development, and workplace culture. Subdomains: Recruitment, Employee Relations, Training and Development, Compensation and Benefits, and HR Analytics."]
    HumanResources,
    #[doc = "Designing and managing salary structures, bonuses, and benefits to attract and retain employees. Subdomains: Salary Benchmarking, Benefits Administration, Incentive Programs, and Retirement Planning."]
    CompensationAndBenefits,
    #[doc = "Maintaining positive relationships between the employer and employees to foster a productive work environment. Subdomains: Conflict Resolution, Employee Engagement, Workplace Culture, and Labor Relations."]
    EmployeeRelations,
    #[doc = "Using data analysis to improve HR decision-making, workforce planning, and employee performance metrics. Subdomains: People Analytics, Predictive HR, Workforce Metrics, and Data-Driven HR Strategies."]
    HrAnalytics,
    #[doc = "Attracting, screening, and selecting qualified candidates for job openings within an organization. Subdomains: Talent Acquisition, Candidate Sourcing, Interviewing Techniques, and Onboarding Processes."]
    Recruitment,
    #[doc = "Providing employees with skills and knowledge to enhance their job performance and career growth. Subdomains: Skills Training, Leadership Development, Career Pathing, and E-Learning Platforms."]
    TrainingAndDevelopment,
    #[doc = "Systems, methods, and technologies used to teach, learn, and foster knowledge development in individuals and communities. Subdomains: E-Learning, Curriculum Design, Learning Management Systems, Educational Technology, and Pedagogy."]
    Education,
    #[doc = "Creating structured educational content and learning experiences for students. Subdomains: Instructional Design, Learning Objectives, Assessment Strategies, and Content Development."]
    CurriculumDesign,
    #[doc = "Delivering educational content and instruction through digital platforms and online courses. Subdomains: Online Course Development, Virtual Classrooms, Interactive Learning Tools, and Mobile Learning."]
    ELearning,
    #[doc = "Integrating digital tools and resources to enhance teaching and learning experiences. Subdomains: EdTech Innovations, Classroom Technology, Digital Content, and Gamification in Education."]
    EducationalTechnology,
    #[doc = "Software platforms for delivering, tracking, and managing educational courses and training programs. Subdomains: User Experience Design, Content Management, Reporting and Analytics, and System Integration."]
    LearningManagementSystems,
    #[doc = "The methods and practices of teaching, focusing on how best to convey knowledge and skills to learners. Subdomains: Teaching Strategies, Student-Centered Learning, Instructional Theory, and Learning Styles."]
    Pedagogy,
    #[doc = "Production of goods, use of automation and technology in manufacturing, and industrial processes to create products on a large scale. Subdomains: Automation, Robotics, Supply Chain Management, Lean Manufacturing, and Process Engineering."]
    IndustrialManufacturing,
    #[doc = "Using technology to perform processes with minimal human intervention. Subdomains: Automated Manufacturing, Control Systems, Industrial IoT, and Process Automation."]
    Automation,
    #[doc = "Methodology focusing on minimizing waste and maximizing efficiency in the production process. Subdomains: Continuous Improvement, Six Sigma, Value Stream Mapping, and Kaizen."]
    LeanManufacturing,
    #[doc = "Designing, implementing, and optimizing industrial processes to improve efficiency and quality. Subdomains: Process Design, Process Optimization, Quality Control, and Safety Engineering."]
    ProcessEngineering,
    #[doc = "Designing and using robots for manufacturing tasks to enhance productivity and precision. Subdomains: Robotic Process Automation, Industrial Robotics, AI and Robotics, and Collaborative Robots."]
    Robotics,
    #[doc = "Coordinating and managing all activities involved in the production and delivery of goods. Subdomains: Inventory Management, Procurement, Logistics Management, and Demand Forecasting."]
    SupplyChainManagement,
    #[doc = "Systems and processes involved in the movement of goods and people, as well as the physical infrastructure supporting them. Subdomains: Logistics, Automotive, Public Transit, Supply Chain, Freight, and Autonomous Vehicles."]
    Transportation,
    #[doc = "The design, development, manufacturing, and marketing of motor vehicles. Subdomains: Vehicle Design, Automotive Engineering, Electric Vehicles, and Vehicle Manufacturing."]
    Automotive,
    #[doc = "Vehicles equipped with technology to navigate and operate without human control. Subdomains: Self-Driving Cars, Autonomous Trucks, Sensor Technology, and Vehicle AI."]
    AutonomousVehicles,
    #[doc = "The transportation of goods in bulk. Subdomains: Freight Forwarding, Cargo Management, Logistics Operations, and Freight Brokerage."]
    Freight,
    #[doc = "The coordination of complex operations involving people, facilities, and supplies. Subdomains: Warehousing, Distribution Management, Transportation Planning, and Reverse Logistics."]
    Logistics,
    #[doc = "Shared transportation services available for the public, such as buses and trains. Subdomains: Urban Transit Planning, Rail Systems, Bus Networks, and Transit Operations."]
    PublicTransit,
    #[doc = "The system of production, processing, and distribution of goods. Subdomains: Supplier Management, Production Scheduling, Inventory Control, and Global Supply Chain."]
    SupplyChain,
    #[doc = "Management, delivery, and innovation of medical services, treatments, and technologies aimed at improving the health and well-being of individuals and populations."]
    Healthcare,
    #[doc = "Systems for managing healthcare data to support clinical and administrative decision-making. Subdomains: Hospital Information Systems, Clinical Decision Support, Health Data Security, and Interoperability Solutions."]
    HealthInformationSystems,
    #[doc = "The management and analysis of healthcare data to improve patient care and operational efficiency. Subdomains: Electronic Health Records, Health Data Analytics, Clinical Informatics, and Health IT Systems."]
    HealthcareInformatics,
    #[doc = "Innovations and devices used to improve the diagnosis, treatment, and management of health conditions. Subdomains: Medical Devices, Diagnostic Equipment, Wearable Health Tech, and Biotech Innovations."]
    MedicalTechnology,
    #[doc = "Software solutions that help healthcare providers manage patient information and clinical processes. Subdomains: Appointment Scheduling, Patient Portals, Billing and Coding, and Health Record Management."]
    PatientManagementSystems,
    #[doc = "The delivery of healthcare services through telecommunications technology. Subdomains: Remote Consultation, Telehealth Platforms, Mobile Health, and Virtual Care."]
    Telemedicine,

}

#[cfg(all(feature = "domain", feature = "identify"))]
pub trait Identify {
    fn uid() -> u32;
    fn name() -> &'static str;
}

#[cfg(all(feature = "domain", feature = "identify"))]
impl Identify for Domain {
    fn uid() -> u32 {
        match Self {
            Domain::Technology => 1,
            Domain::Security => 107,
            Domain::CommunicationSystems => 108,
            Domain::InformationTechnology => 106,
            Domain::InternetOfThings => 101,
            Domain::NetworkManagement => 105,
            Domain::NetworkOperations => 104,
            Domain::Networking => 103,
            Domain::SoftwareEngineering => 102,
            Domain::FinanceAndBusiness => 2,
            Domain::Banking => 201,
            Domain::ConsumerGoods => 204,
            Domain::Finance => 202,
            Domain::InvestmentServices => 203,
            Domain::Retail => 205,
            Domain::LifeScience => 3,
            Domain::Bioinformatics => 304,
            Domain::Biotechnology => 301,
            Domain::Genomics => 303,
            Domain::MolecularBiology => 305,
            Domain::PharmaceuticalResearch => 302,
            Domain::TrustAndSafety => 4,
            Domain::ContentModeration => 402,
            Domain::DataPrivacy => 404,
            Domain::FraudPrevention => 403,
            Domain::OnlineSafety => 401,
            Domain::RiskManagement => 405,
            Domain::HumanResources => 5,
            Domain::CompensationAndBenefits => 504,
            Domain::EmployeeRelations => 502,
            Domain::HrAnalytics => 505,
            Domain::Recruitment => 501,
            Domain::TrainingAndDevelopment => 503,
            Domain::Education => 6,
            Domain::CurriculumDesign => 602,
            Domain::ELearning => 601,
            Domain::EducationalTechnology => 604,
            Domain::LearningManagementSystems => 603,
            Domain::Pedagogy => 605,
            Domain::IndustrialManufacturing => 7,
            Domain::Automation => 701,
            Domain::LeanManufacturing => 704,
            Domain::ProcessEngineering => 705,
            Domain::Robotics => 702,
            Domain::SupplyChainManagement => 703,
            Domain::Transportation => 8,
            Domain::Automotive => 802,
            Domain::AutonomousVehicles => 806,
            Domain::Freight => 805,
            Domain::Logistics => 801,
            Domain::PublicTransit => 803,
            Domain::SupplyChain => 804,
            Domain::Healthcare => 9,
            Domain::HealthInformationSystems => 905,
            Domain::HealthcareInformatics => 903,
            Domain::MedicalTechnology => 901,
            Domain::PatientManagementSystems => 904,
            Domain::Telemedicine => 902,

        }
    }
    fn name() -> &'static str {
        match Self {
            Domain::Technology => "technology",
            Domain::Security => "technology/security",
            Domain::CommunicationSystems => "technology/communication_systems",
            Domain::InformationTechnology => "technology/information_technology",
            Domain::InternetOfThings => "technology/internet_of_things",
            Domain::NetworkManagement => "technology/network_management",
            Domain::NetworkOperations => "technology/network_operations",
            Domain::Networking => "technology/networking",
            Domain::SoftwareEngineering => "technology/software_engineering",
            Domain::FinanceAndBusiness => "finance_and_business",
            Domain::Banking => "finance_and_business/banking",
            Domain::ConsumerGoods => "finance_and_business/consumer_goods",
            Domain::Finance => "finance_and_business/finance",
            Domain::InvestmentServices => "finance_and_business/investment_services",
            Domain::Retail => "finance_and_business/retail",
            Domain::LifeScience => "life_science",
            Domain::Bioinformatics => "life_science/bioinformatics",
            Domain::Biotechnology => "life_science/biotechnology",
            Domain::Genomics => "life_science/genomics",
            Domain::MolecularBiology => "life_science/molecular_biology",
            Domain::PharmaceuticalResearch => "life_science/pharmaceutical_research",
            Domain::TrustAndSafety => "trust_and_safety",
            Domain::ContentModeration => "trust_and_safety/content_moderation",
            Domain::DataPrivacy => "trust_and_safety/data_privacy",
            Domain::FraudPrevention => "trust_and_safety/fraud_prevention",
            Domain::OnlineSafety => "trust_and_safety/online_safety",
            Domain::RiskManagement => "trust_and_safety/risk_management",
            Domain::HumanResources => "human_resources",
            Domain::CompensationAndBenefits => "human_resources/compensation_and_benefits",
            Domain::EmployeeRelations => "human_resources/employee_relations",
            Domain::HrAnalytics => "human_resources/hr_analytics",
            Domain::Recruitment => "human_resources/recruitment",
            Domain::TrainingAndDevelopment => "human_resources/training_and_development",
            Domain::Education => "education",
            Domain::CurriculumDesign => "education/curriculum_design",
            Domain::ELearning => "education/e_learning",
            Domain::EducationalTechnology => "education/educational_technology",
            Domain::LearningManagementSystems => "education/learning_management_systems",
            Domain::Pedagogy => "education/pedagogy",
            Domain::IndustrialManufacturing => "industrial_manufacturing",
            Domain::Automation => "industrial_manufacturing/automation",
            Domain::LeanManufacturing => "industrial_manufacturing/lean_manufacturing",
            Domain::ProcessEngineering => "industrial_manufacturing/process_engineering",
            Domain::Robotics => "industrial_manufacturing/robotics",
            Domain::SupplyChainManagement => "industrial_manufacturing/supply_chain_management",
            Domain::Transportation => "transportation",
            Domain::Automotive => "transportation/automotive",
            Domain::AutonomousVehicles => "transportation/autonomous_vehicles",
            Domain::Freight => "transportation/freight",
            Domain::Logistics => "transportation/logistics",
            Domain::PublicTransit => "transportation/public_transit",
            Domain::SupplyChain => "transportation/supply_chain",
            Domain::Healthcare => "healthcare",
            Domain::HealthInformationSystems => "healthcare/health_information_systems",
            Domain::HealthcareInformatics => "healthcare/healthcare_informatics",
            Domain::MedicalTechnology => "healthcare/medical_technology",
            Domain::PatientManagementSystems => "healthcare/patient_management_systems",
            Domain::Telemedicine => "healthcare/telemedicine",

        }
    }
}

#[cfg(feature = "domain")]
impl From<Domain> for &str {
    fn from(value: Domain) -> &'static str {
        match value {
            Domain::Technology => "technology",
            Domain::Security => "technology/security",
            Domain::CommunicationSystems => "technology/communication_systems",
            Domain::InformationTechnology => "technology/information_technology",
            Domain::InternetOfThings => "technology/internet_of_things",
            Domain::NetworkManagement => "technology/network_management",
            Domain::NetworkOperations => "technology/network_operations",
            Domain::Networking => "technology/networking",
            Domain::SoftwareEngineering => "technology/software_engineering",
            Domain::FinanceAndBusiness => "finance_and_business",
            Domain::Banking => "finance_and_business/banking",
            Domain::ConsumerGoods => "finance_and_business/consumer_goods",
            Domain::Finance => "finance_and_business/finance",
            Domain::InvestmentServices => "finance_and_business/investment_services",
            Domain::Retail => "finance_and_business/retail",
            Domain::LifeScience => "life_science",
            Domain::Bioinformatics => "life_science/bioinformatics",
            Domain::Biotechnology => "life_science/biotechnology",
            Domain::Genomics => "life_science/genomics",
            Domain::MolecularBiology => "life_science/molecular_biology",
            Domain::PharmaceuticalResearch => "life_science/pharmaceutical_research",
            Domain::TrustAndSafety => "trust_and_safety",
            Domain::ContentModeration => "trust_and_safety/content_moderation",
            Domain::DataPrivacy => "trust_and_safety/data_privacy",
            Domain::FraudPrevention => "trust_and_safety/fraud_prevention",
            Domain::OnlineSafety => "trust_and_safety/online_safety",
            Domain::RiskManagement => "trust_and_safety/risk_management",
            Domain::HumanResources => "human_resources",
            Domain::CompensationAndBenefits => "human_resources/compensation_and_benefits",
            Domain::EmployeeRelations => "human_resources/employee_relations",
            Domain::HrAnalytics => "human_resources/hr_analytics",
            Domain::Recruitment => "human_resources/recruitment",
            Domain::TrainingAndDevelopment => "human_resources/training_and_development",
            Domain::Education => "education",
            Domain::CurriculumDesign => "education/curriculum_design",
            Domain::ELearning => "education/e_learning",
            Domain::EducationalTechnology => "education/educational_technology",
            Domain::LearningManagementSystems => "education/learning_management_systems",
            Domain::Pedagogy => "education/pedagogy",
            Domain::IndustrialManufacturing => "industrial_manufacturing",
            Domain::Automation => "industrial_manufacturing/automation",
            Domain::LeanManufacturing => "industrial_manufacturing/lean_manufacturing",
            Domain::ProcessEngineering => "industrial_manufacturing/process_engineering",
            Domain::Robotics => "industrial_manufacturing/robotics",
            Domain::SupplyChainManagement => "industrial_manufacturing/supply_chain_management",
            Domain::Transportation => "transportation",
            Domain::Automotive => "transportation/automotive",
            Domain::AutonomousVehicles => "transportation/autonomous_vehicles",
            Domain::Freight => "transportation/freight",
            Domain::Logistics => "transportation/logistics",
            Domain::PublicTransit => "transportation/public_transit",
            Domain::SupplyChain => "transportation/supply_chain",
            Domain::Healthcare => "healthcare",
            Domain::HealthInformationSystems => "healthcare/health_information_systems",
            Domain::HealthcareInformatics => "healthcare/healthcare_informatics",
            Domain::MedicalTechnology => "healthcare/medical_technology",
            Domain::PatientManagementSystems => "healthcare/patient_management_systems",
            Domain::Telemedicine => "healthcare/telemedicine",

        }
    }
}

#[cfg(feature = "domain")]
impl From<Domain> for u32 {
    fn from(value: Domain) -> u32 {
        match value {
            Domain::Technology => 1,
            Domain::Security => 107,
            Domain::CommunicationSystems => 108,
            Domain::InformationTechnology => 106,
            Domain::InternetOfThings => 101,
            Domain::NetworkManagement => 105,
            Domain::NetworkOperations => 104,
            Domain::Networking => 103,
            Domain::SoftwareEngineering => 102,
            Domain::FinanceAndBusiness => 2,
            Domain::Banking => 201,
            Domain::ConsumerGoods => 204,
            Domain::Finance => 202,
            Domain::InvestmentServices => 203,
            Domain::Retail => 205,
            Domain::LifeScience => 3,
            Domain::Bioinformatics => 304,
            Domain::Biotechnology => 301,
            Domain::Genomics => 303,
            Domain::MolecularBiology => 305,
            Domain::PharmaceuticalResearch => 302,
            Domain::TrustAndSafety => 4,
            Domain::ContentModeration => 402,
            Domain::DataPrivacy => 404,
            Domain::FraudPrevention => 403,
            Domain::OnlineSafety => 401,
            Domain::RiskManagement => 405,
            Domain::HumanResources => 5,
            Domain::CompensationAndBenefits => 504,
            Domain::EmployeeRelations => 502,
            Domain::HrAnalytics => 505,
            Domain::Recruitment => 501,
            Domain::TrainingAndDevelopment => 503,
            Domain::Education => 6,
            Domain::CurriculumDesign => 602,
            Domain::ELearning => 601,
            Domain::EducationalTechnology => 604,
            Domain::LearningManagementSystems => 603,
            Domain::Pedagogy => 605,
            Domain::IndustrialManufacturing => 7,
            Domain::Automation => 701,
            Domain::LeanManufacturing => 704,
            Domain::ProcessEngineering => 705,
            Domain::Robotics => 702,
            Domain::SupplyChainManagement => 703,
            Domain::Transportation => 8,
            Domain::Automotive => 802,
            Domain::AutonomousVehicles => 806,
            Domain::Freight => 805,
            Domain::Logistics => 801,
            Domain::PublicTransit => 803,
            Domain::SupplyChain => 804,
            Domain::Healthcare => 9,
            Domain::HealthInformationSystems => 905,
            Domain::HealthcareInformatics => 903,
            Domain::MedicalTechnology => 901,
            Domain::PatientManagementSystems => 904,
            Domain::Telemedicine => 902,

        }
    }
}