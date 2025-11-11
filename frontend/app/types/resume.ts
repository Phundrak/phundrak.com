export interface ResumeExperience extends TimelineItem {
  tools: string[];
}

export interface ResumeContent {
  experience: ResumeExperience[];
  education: TimelineItem[];
  otherTools: string[];
  devops: string[];
  os: string[];
  programmingLanguages: string[];
  frameworks: string[];
}
