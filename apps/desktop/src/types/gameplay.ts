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

export type StructuredGameplayAction =
  | { type: 'SEND_MESSAGE'; recipientId: string; text: string }
  | { type: 'APPLY_FOR_JOB'; jobId: string; companyId: string; title: string; companyName: string }
  | { type: 'REGISTER_COMPANY'; name: string; structure: string; partners: string[]; authorizedCapital: number }
  | { type: 'TRAVEL'; destinationCityId: string; transportMode: string; stayDays: number };
