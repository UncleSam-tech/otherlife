import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { LifeNav } from './components/LifeNav';
import { LifeFeed, FeedEvent } from './components/LifeFeed';
import { NowSidebar, SidebarStateData } from './components/NowSidebar';
import { ActionPromptBar } from './components/ActionPromptBar';
import { CausalityInspector } from './components/CausalityInspector';
import { CreationWizard, NewLifeFormState } from './components/creator/CreationWizard';

import { EducationView } from './components/views/EducationView';
import { CareerView } from './components/views/CareerView';
import { HealthView } from './components/views/HealthView';
import { FamilyRomanceView } from './components/views/FamilyRomanceView';
import { WorldNewsView } from './components/views/WorldNewsView';
import { BiographyView } from './components/views/BiographyView';
import { FootballView } from './components/views/FootballView';
import { BusinessEconomyView } from './components/views/BusinessEconomyView';
import { PoliticsView } from './components/views/PoliticsView';
import { EntertainmentMediaView } from './components/views/EntertainmentMediaView';
import { CrimeUnderworldView } from './components/views/CrimeUnderworldView';
import { ScienceTechView } from './components/views/ScienceTechView';
import { BeliefReligionView } from './components/views/BeliefReligionView';
import { GlobalTravelView } from './components/views/GlobalTravelView';
import { MilitaryWarView } from './components/views/MilitaryWarView';
import { HealthcareMedicineView } from './components/views/HealthcareMedicineView';
import { SocialMediaDigitalView } from './components/views/SocialMediaDigitalView';
import { EnvironmentNatureView } from './components/views/EnvironmentNatureView';
import { SecretSocietyView } from './components/views/SecretSocietyView';
import { SpaceExplorationView } from './components/views/SpaceExplorationView';
import { TranshumanismCyberneticsView } from './components/views/TranshumanismCyberneticsView';
import { PostScarcityCosmicLegacyView } from './components/views/PostScarcityCosmicLegacyView';

import './styles/globals.css';

async function callTauriCommand<T>(cmd: string, args: Record<string, any> = {}): Promise<T | null> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[Tauri Fallback]: ${cmd} fallback mode executed`, err);
    return null;
  }
}

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState('overview');
  const [devMode, setDevMode] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [showWizard, setShowWizard] = useState(false);
  const [inspectingEvent, setInspectingEvent] = useState<FeedEvent | null>(null);

  const [registries, setRegistries] = useState<any>(null);

  const [gameState, setGameState] = useState({
    timeFormatted: '12 OCT 2029 · 16:30',
    age: 14,
    cash: 24,
    location: 'city:real:glasgow',
    playerName: 'James Morrison',
    interests: ['football'],
    goals: ['play_pro_football'],
    lifeStage: 'Adolescence',
    maritalStatus: 'Single',
    jobTitle: 'Unemployed / Student',
    monthlySalary: 0,
    housingType: 'FamilyHome',
    fitness: 75,
    stress: 20,
  });

  const [suggestions, setSuggestions] = useState<string[]>([
    "Tell Mum I'm going to James's house to study math, but secretly go to football training.",
    "Spend the evening studying math to improve grades and rebuild Mum's trust.",
    "Attend official Saturday youth match and showcase skills for the scout.",
  ]);

  const [sidebarData, setSidebarData] = useState<SidebarStateData>({
    commitments: [
      { title: 'Saturday Youth Match', description: 'Regional scout attending match.', urgency: 'HIGH' },
    ],
    household_trust: 0.75,
    household_resentment: 0.15,
    active_interest: 'football',
    primary_skill_name: 'football_control',
    primary_skill_value: 70.0,
    life_stage: 'Adolescence',
    marital_status: 'Single',
    job_title: 'Unemployed / Student',
    monthly_salary: 0,
    fitness: 75,
    stress: 20,
  });

  const [events, setEvents] = useState<FeedEvent[]>([
    {
      id: 'init-1',
      timestamp: '12 OCT 2029 · 09:00',
      eventType: 'TIMELINE_START',
      summary: 'You initialized your alternate life timeline.',
      causalityNote: 'Initial scenario seeded.',
    },
  ]);

  useEffect(() => {
    callTauriCommand<any>('get_registries').then((res) => {
      if (res) {
        setRegistries(res);
      } else {
        setRegistries({
          countries: [{ id: 'country:real:united_kingdom', name: 'United Kingdom', currency_symbol: '£' }, { id: 'country:real:nigeria', name: 'Nigeria', currency_symbol: '₦' }],
          locations: [{ id: 'city:real:glasgow', name: 'Glasgow', region_name: 'Scotland', country_id: 'country:real:united_kingdom' }, { id: 'city:real:lagos', name: 'Lagos', region_name: 'Lagos State', country_id: 'country:real:nigeria' }],
          skills: [{ id: 'football_control', name: 'Football Control', category: 'Sport', description: 'Ball control' }, { id: 'singing', name: 'Vocal Performance', category: 'Creative', description: 'Singing' }],
          traits: [{ id: 'ambition', name: 'Ambition' }],
          interests: [{ id: 'football', name: 'Football' }, { id: 'music', name: 'Music' }],
          goals: [{ id: 'play_pro_football', name: 'Play Pro Football' }, { id: 'become_musician', name: 'Become Musician' }],
        });
      }
    });
  }, []);

  const handleStartCustomLife = async (formState: NewLifeFormState) => {
    setShowWizard(false);
    setIsLoading(true);

    const configPayload = {
      creation_mode: 'CUSTOM',
      starting_year: formState.startingYear,
      country_id: formState.countryId,
      location_id: formState.locationId,
      starting_age: formState.startingAge,
      first_name: formState.firstName,
      last_name: formState.lastName,
      sex: formState.sex,
      household_income_tier: formState.householdIncomeTier,
      traits: formState.traits,
      skills: formState.skills,
      interests: formState.interests,
      goals: formState.goals,
    };

    const res = await callTauriCommand<[any, string[], any]>('start_new_life', { config: configPayload, seed: Date.now() % 100000 });

    if (res && res[0]) {
      const [dto, suggs, sbar] = res;
      setGameState({
        timeFormatted: dto.time_formatted,
        age: dto.age,
        cash: dto.cash,
        location: dto.location,
        playerName: dto.player_name,
        interests: dto.interests,
        goals: dto.goals,
        lifeStage: dto.life_stage,
        maritalStatus: dto.marital_status,
        jobTitle: dto.job_title,
        monthlySalary: dto.monthly_salary,
        housingType: dto.housing_type,
        fitness: dto.fitness,
        stress: dto.stress,
      });
      setSuggestions(suggs);
      setSidebarData(sbar);
      setEvents([
        {
          id: String(Date.now()),
          timestamp: dto.time_formatted,
          eventType: 'NEW_LIFE',
          summary: `You began your alternate life as ${dto.player_name} in ${dto.location.replace('city:real:', '').toUpperCase()}.`,
          causalityNote: 'New Life created with custom parameters.',
        },
      ]);
    } else {
      setGameState({
        timeFormatted: `01 OCT ${formState.startingYear} · 09:00`,
        age: formState.startingAge,
        cash: formState.householdIncomeTier === 'HIGH' ? 2500 : 150,
        location: formState.locationId,
        playerName: `${formState.firstName} ${formState.lastName}`,
        interests: formState.interests,
        goals: formState.goals,
        lifeStage: formState.startingAge < 18 ? 'Adolescence' : 'Adulthood',
        maritalStatus: 'Single',
        jobTitle: 'Unemployed',
        monthlySalary: 0,
        housingType: formState.startingAge < 18 ? 'FamilyHome' : 'Renting',
        fitness: 75,
        stress: 20,
      });

      setSidebarData({
        commitments: [{ title: 'Daily Life', description: 'Explore your starting environment.', urgency: 'LOW' }],
        household_trust: 0.8,
        household_resentment: 0.1,
        active_interest: formState.interests[0] || 'General Life',
        primary_skill_name: Object.keys(formState.skills)[0] || 'communication',
        primary_skill_value: Object.values(formState.skills)[0] || 50,
        life_stage: formState.startingAge < 18 ? 'Adolescence' : 'Adulthood',
        marital_status: 'Single',
        job_title: 'Unemployed',
        monthly_salary: 0,
        fitness: 75,
        stress: 20,
      });

      setSuggestions([
        `Explore ${formState.locationId.replace('city:real:', '').toUpperCase()} neighborhood.`,
        `Practice your ${Object.keys(formState.skills)[0] || 'skills'}.`,
        'Talk to family about future goals.',
      ]);

      setEvents([
        {
          id: String(Date.now()),
          timestamp: `01 OCT ${formState.startingYear} · 09:00`,
          eventType: 'NEW_LIFE',
          summary: `You began your custom life as ${formState.firstName} ${formState.lastName} (Age ${formState.startingAge}) in ${formState.locationId.replace('city:real:', '').toUpperCase()}.`,
          causalityNote: 'Custom starting conditions configured.',
        },
      ]);
    }

    setIsLoading(false);
  };

  const handleSubmitAction = async (inputText: string) => {
    setIsLoading(true);

    const res = await callTauriCommand<[any, any, string[], any]>('submit_player_action', { inputText });

    if (res && res[0] && res[1]) {
      const [dto, stepRes, suggs, sbar] = res;
      setGameState({
        timeFormatted: dto.time_formatted,
        age: dto.age,
        cash: dto.cash,
        location: dto.location,
        playerName: dto.player_name,
        interests: dto.interests,
        goals: dto.goals,
        lifeStage: dto.life_stage,
        maritalStatus: dto.marital_status,
        jobTitle: dto.job_title,
        monthlySalary: dto.monthly_salary,
        housingType: dto.housing_type,
        fitness: dto.fitness,
        stress: dto.stress,
      });

      setSuggestions(suggs);
      setSidebarData(sbar);

      const newEv: FeedEvent = {
        id: stepRes.event_record.id,
        timestamp: stepRes.event_record.timestamp,
        eventType: stepRes.event_record.event_type,
        summary: stepRes.narrative,
        causalityNote: stepRes.causality_note,
        success: stepRes.success,
      };

      setEvents((prev) => [newEv, ...prev]);
    } else {
      setTimeout(() => {
        const newEv: FeedEvent = {
          id: String(Date.now()),
          timestamp: gameState.timeFormatted,
          eventType: 'ACTION',
          summary: `You executed: "${inputText}". The world reacted accordingly.`,
          causalityNote: 'Action resolved via engine tick.',
          success: true,
        };
        setEvents((prev) => [newEv, ...prev]);
      }, 250);
    }

    setIsLoading(false);
  };

  const renderMainViewContent = () => {
    switch (activeTab) {
      case 'education':
        return (
          <EducationView
            gradeLevel={gameState.age > 5 && gameState.age < 18 ? gameState.age - 5 : 0}
            academicPerformance={65}
            qualifications={[]}
            onStudy={() => handleSubmitAction('Spend 2 hours studying and reviewing concepts.')}
          />
        );
      case 'career':
        return (
          <CareerView
            jobTitle={gameState.jobTitle}
            monthlySalary={gameState.monthlySalary}
            onApplyJob={() => handleSubmitAction('Apply for a part-time job vacancy.')}
            onWorkShift={() => handleSubmitAction('Work a shift to earn salary and build career experience.')}
          />
        );
      case 'finances':
      case 'business':
      case 'economy':
        return (
          <BusinessEconomyView
            economicCycle="GROWTH"
            inflationRate={0.025}
            interestRate={0.045}
            playerCash={gameState.cash}
            onFoundBusiness={() => handleSubmitAction('Found a new business venture with initial seed capital.')}
          />
        );
      case 'health':
        return (
          <HealthView
            fitness={gameState.fitness}
            stress={gameState.stress}
            onSeekMedicalTreatment={() => handleSubmitAction('Seek medical treatment and rest to reduce stress.')}
          />
        );
      case 'people':
      case 'family':
        return (
          <FamilyRomanceView
            maritalStatus={gameState.maritalStatus}
            onDate={() => handleSubmitAction('Go on a date to meet new romantic partners.')}
            onMarry={() => handleSubmitAction('Propose marriage to your partner.')}
            onDivorce={() => handleSubmitAction('Finalize divorce proceedings.')}
            onHaveChild={() => handleSubmitAction('Discuss starting a family and having a child.')}
          />
        );
      case 'world':
      case 'news':
        return (
          <WorldNewsView
            newsItems={[
              {
                id: 'news-1',
                timestamp: gameState.timeFormatted,
                headline: 'City Economic Development Digest Published',
                body: 'Municipal authorities released quarterly forecasts on employment and regional investment.',
                category: 'LOCAL',
              },
            ]}
          />
        );
      case 'football':
        return (
          <FootballView
            footballRole="Academy Prospect"
            clubName="Celtic FC"
            weeklyWage={450}
            ballControl={70}
            pace={72}
            stamina={68}
            onTrain={() => handleSubmitAction('Attend training session to hone ball control and physical conditioning.')}
            onPlayMatch={() => handleSubmitAction('Play official saturday match.')}
          />
        );
      case 'politics':
      case 'power':
        return (
          <PoliticsView
            partyName="Labour Party"
            officeTitle="Member of Parliament (MP)"
            isCampaigning={true}
            pollingPct={48.5}
            onLaunchCampaign={() => handleSubmitAction('Launch official election campaign for Member of Parliament.')}
            onHoldRally={() => handleSubmitAction('Host campaign rally and constituency townhall.')}
          />
        );
      case 'entertainment':
      case 'media':
        return (
          <EntertainmentMediaView
            fameLevel={32.5}
            publicReputation={88.0}
            fanbaseCount={24500}
            onProduceRelease={() => handleSubmitAction('Produce and record a new creative album in studio.')}
            onPromoteMedia={() => handleSubmitAction('Participate in media press interview and promotional campaign.')}
          />
        );
      case 'crime':
      case 'underworld':
        return (
          <CrimeUnderworldView
            legalStatus="Clean"
            criminalRecordCount={0}
            onCommitCrime={(crimeType) => handleSubmitAction(`Commit ${crimeType} theft operation in city.`)}
            onHireDefenseLawyer={() => handleSubmitAction('Retain senior criminal defense attorney.')}
          />
        );
      case 'science':
      case 'tech':
        return (
          <ScienceTechView
            degreeCount={1}
            publishedPapersCount={2}
            patentsCount={1}
            onEnrollProgram={() => handleSubmitAction('Enroll in PhD program in Artificial Intelligence at University.')}
            onLaunchResearch={() => handleSubmitAction('Launch scientific research experiment project in laboratory.')}
          />
        );
      case 'religion':
      case 'belief':
        return (
          <BeliefReligionView
            faithName="Secular Humanism"
            devotionLevel={42.0}
            tithesDonated={350.0}
            spiritualRank="LAITY"
            onAttendWorship={() => handleSubmitAction('Attend community worship and philosophical reflection service.')}
            onDonateTithe={() => handleSubmitAction('Donate £50 tithe offering to congregation treasury.')}
            onFoundMovement={() => handleSubmitAction('Found a new philosophical faith movement.')}
          />
        );
      case 'travel':
      case 'immigration':
        return (
          <GlobalTravelView
            currentLocation="Glasgow, United Kingdom"
            passportCount={1}
            visaCount={1}
            travelCount={3}
            onBookFlight={() => handleSubmitAction('Book international flight ticket to Tokyo, Japan.')}
            onApplyPassport={() => handleSubmitAction('Apply for official national passport renewal.')}
          />
        );
      case 'military':
      case 'war':
        return (
          <MilitaryWarView
            branch="ARMY"
            rank="LIEUTENANT"
            yearsServed={4}
            combatDeployments={2}
            isActiveDuty={true}
            isVeteran={false}
            pensionMonthly={1200.0}
            onEnlist={(b) => handleSubmitAction(`Enlist in Armed Forces branch ${b}.`)}
            onPromoteRank={() => handleSubmitAction('Submit application for military rank promotion.')}
            onDeployCombat={() => handleSubmitAction('Deploy with unit to active combat zone in peacekeeping operation.')}
            onDischargeVeteran={() => handleSubmitAction('Apply for honorable discharge from military service to become veteran.')}
          />
        );
      case 'healthcare':
      case 'medicine':
        return (
          <HealthcareMedicineView
            fitness={78.5}
            stress={18.0}
            conditionsCount={0}
            surgeriesCount={1}
            hasWill={true}
            onUndergoSurgery={() => handleSubmitAction('Schedule and undergo elective orthopedic surgery.')}
            onDraftWill={() => handleSubmitAction('Draft Will & Testament specifying family estate beneficiaries.')}
            onQuarantineCheck={() => handleSubmitAction('Check regional epidemic public health advisory notice.')}
          />
        );
      case 'digital':
      case 'social_media':
        return (
          <SocialMediaDigitalView
            platform="YOUTUBE"
            handle="alexmorgan_official"
            followers={85400}
            influencerTier="MICRO"
            postsCount={12}
            onCreateAccount={() => handleSubmitAction('Create new social media creator profile on YouTube.')}
            onPostContent={() => handleSubmitAction('Publish digital video vlog post to subscribers.')}
            onAcceptSponsorship={() => handleSubmitAction('Accept £2,500 brand sponsorship commercial deal.')}
            onCyberSecurityAudit={() => handleSubmitAction('Perform cybersecurity security audit on digital profiles.')}
          />
        );
      case 'environment':
      case 'nature':
        return (
          <EnvironmentNatureView
            season="SUMMER"
            condition="HEATWAVE"
            temperatureCelsius={26.5}
            airQualityIndex={42}
            activeDisastersCount={0}
            onSimulateWeather={() => handleSubmitAction('Simulate seasonal weather shift and regional temperature update.')}
            onTriggerDisaster={() => handleSubmitAction('Issue emergency natural disaster alert for regional area.')}
            onRebuildInfrastructure={() => handleSubmitAction('Allocate £50,000 relief funding to rebuild infrastructure.')}
          />
        );
      case 'secret_society':
      case 'subculture':
        return (
          <SecretSocietyView
            societyName="Order of the Silver Hand"
            societyType="TEMPLAR_LODGE"
            rank="ADEPT"
            covertReputation={68.5}
            membershipsCount={1}
            operationsCount={3}
            onJoinSociety={() => handleSubmitAction('Initiate entry into secret society with encrypted password cipher.')}
            onPerformRitual={() => handleSubmitAction('Perform esoteric occult ritual in subterranean lodge chamber.')}
            onLaunchOperation={() => handleSubmitAction('Launch covert intelligence operation targeting municipal office.')}
            onAdvanceRank={() => handleSubmitAction('Advance member rank to Grand Master in secret society hierarchy.')}
          />
        );
      case 'space':
      case 'exploration':
        return (
          <SpaceExplorationView
            agencyName="Aetheria Aerospace"
            agencyType="PRIVATE_AEROSPACE"
            missionsCount={2}
            satellitesCount={4}
            patentsCount={1}
            reputation={88.0}
            onFundAgency={() => handleSubmitAction('Fund private aerospace agency with £100,000 seed capital.')}
            onLaunchMission={() => handleSubmitAction('Launch robotic Mars Rover planetary mission into transfer orbit.')}
            onDeploySatellite={() => handleSubmitAction('Deploy commercial communications satellite into Low Earth Orbit.')}
            onRegisterPatent={() => handleSubmitAction('Register aerospace rocket propulsion patent with IPO office.')}
          />
        );
      case 'transhumanism':
      case 'cybernetics':
        return (
          <TranshumanismCyberneticsView
            implantsCount={2}
            mindUploadsCount={1}
            digitalAvatarName="Avatar-Nexus-01"
            substrate="QUANTUM_CORE"
            fidelity={99.8}
            onInstallImplant={() => handleSubmitAction('Install Neural Link Interface cybernetic implant.')}
            onUploadMind={() => handleSubmitAction('Upload consciousness into digital avatar cloud substrate.')}
            onUpgradeSubstrate={() => handleSubmitAction('Upgrade digital avatar mind substrate to Quantum Core.')}
          />
        );
      case 'post_scarcity':
      case 'cosmic_legacy':
        return (
          <PostScarcityCosmicLegacyView
            ubdAmount={5000}
            automationIndex={96.5}
            megastructuresCount={1}
            interstellarColoniesCount={3}
            kardashevTier="TYPE_II"
            onDistributeUBD={() => handleSubmitAction('Distribute universal basic dividend of £5,000 to all citizens.')}
            onBuildMegastructure={() => handleSubmitAction('Construct stellar Dyson Swarm megastructure for energy capture.')}
            onEstablishColony={() => handleSubmitAction('Establish new interstellar colony in Alpha Centauri star system.')}
            onEvaluateLegacy={() => handleSubmitAction('Evaluate multi-generational galactic legacy and Kardashev rating.')}
          />
        );
      case 'goals':
      case 'biography':
        return (
          <BiographyView
            biographyText={`# The Life Story of ${gameState.playerName}\n\n- **${gameState.timeFormatted}**: Commenced timeline in ${gameState.location.replace('city:real:', '').toUpperCase()}.\n- **Active Interest**: ${gameState.interests[0] || 'General Life'}.\n- **Current Standing**: Age ${gameState.age} (${gameState.lifeStage}), Cash: £${gameState.cash.toFixed(2)}.`}
          />
        );
      default:
        return (
          <LifeFeed
            events={events}
            onInspectCausality={(ev) => setInspectingEvent(ev)}
            devMode={devMode}
          />
        );
    }
  };

  return (
    <div className="app-layout">
      <Header
        timeFormatted={gameState.timeFormatted}
        age={gameState.age}
        cash={gameState.cash}
        location={gameState.location}
        playerName={gameState.playerName}
        devMode={devMode}
        onToggleDevMode={() => setDevMode(!devMode)}
        onOpenNewLifeWizard={() => setShowWizard(true)}
      />

      <div className="main-viewport">
        <LifeNav
          activeTab={activeTab}
          onSelectTab={setActiveTab}
          interests={gameState.interests}
        />
        <div style={{ flex: 1, height: '100%', overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
          {renderMainViewContent()}
        </div>
        <NowSidebar
          sidebarData={sidebarData}
          devMode={devMode}
        />
      </div>

      <ActionPromptBar
        onSubmitAction={handleSubmitAction}
        isLoading={isLoading}
        suggestions={suggestions}
      />

      <CausalityInspector
        event={inspectingEvent}
        onClose={() => setInspectingEvent(null)}
      />

      {showWizard && (
        <CreationWizard
          registries={registries}
          onClose={() => setShowWizard(false)}
          onSubmitNewLife={handleStartCustomLife}
        />
      )}
    </div>
  );
};
