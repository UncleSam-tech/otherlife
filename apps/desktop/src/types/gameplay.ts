export interface PhoneMessageDTO {
  id: string;
  sender_id: string;
  sender_name: string;
  recipient_id: string;
  text: string;
  timestamp: string;
  is_read: boolean;
  is_delivered: boolean;
}

export interface LetterNotificationDTO {
  id: string;
  sender: string;
  subject: string;
  body: string;
  is_read: boolean;
  date_received: string;
}

export interface WorldMapPlaceDTO {
  id: string;
  name: string;
  category: string;
  district_name: string;
  description: string;
  map_x: number;
  map_y: number;
  travel_minutes: number;
  travel_cost: number;
  is_current: boolean;
  is_open: boolean;
  present_people_count: number;
}

export type StructuredGameplayAction =
  | { type: 'SEND_MESSAGE'; recipientId: string; text: string }
  | { type: 'COMMUTE'; placeId: string; transportMode: string }
  | { type: 'CONVERSE'; npcId: string; dialogue: string }
  | { type: 'BUSINESS_OPERATION'; companyName: string; operation: string; plan: string }
  | {
      type: 'UNIVERSITY_APPLICATION';
      institution: string;
      degreeProgram: string;
      primaryCourse: string;
      studyMode: string;
      fundingPlan: string;
    }
  | {
      type: 'APPLY_FOR_JOB';
      jobId: string;
      companyId: string;
      title: string;
      companyName: string;
      resumeSummary: string;
      coverLetter: string;
      availability: string;
    }
  | {
      type: 'REGISTER_COMPANY';
      name: string;
      structure: string;
      partners: string[];
      authorizedCapital: number;
      businessActivity: string;
      registeredAddress: string;
    }
  | {
      type: 'TRAVEL';
      destinationCityId: string;
      transportMode: string;
      stayDays: number;
      operatorName: string;
      serviceClass: string;
      fare: number;
      accommodation: string;
      departureTiming: string;
      journeyType: string;
      immigrationPathway: string;
    };
