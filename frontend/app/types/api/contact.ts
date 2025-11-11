export interface ContactRequest {
  name: string;
  email: string;
  message: string;
  website?: string | null;
}

export interface ContactResponse {
  success: boolean;
  message: string;
}
