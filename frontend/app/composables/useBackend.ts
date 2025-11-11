export const useBackend = () => {
  const api = useApi();

  const getMeta = () => api.get<MetaResponse>('/meta');
  const postContact = (contact: ContactRequest) => api.post<ContactRequest, ContactResponse>('/contact', contact);

  return { getMeta, postContact };
};
