import Service from "@/types/service.type";

const fetcher = {
  post: (url: string, data: any) => {
    return fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }).then((response) => response.json());
  },

  get: (url: string) => {
    return fetch(url).then((response) => response.json());
  },
};

export const Services = {
  createService: (service: Service) => {
    return fetcher.post("/api/service", service);
  },

  getService: (routeId: string) => {
    return fetcher.get(`/api/service/${routeId}`);
  },

  getServiceList: () => {
    return fetcher.get(`/api/service`);
  },
};
