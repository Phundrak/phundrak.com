import { Observable, of } from 'rxjs';

const cacheAgeLimitInMilliseconds = 1000 * 60 * 60;

export function isDataOutdated(name: string): boolean {
  const lastUpdated: number = +localStorage.getItem(name + '-timestamp');
  const now: number = Date.now();
  const elapsedTime: number = now - lastUpdated;
  return elapsedTime > cacheAgeLimitInMilliseconds;
}

export function storeInCache<T>(
  data: Observable<T>,
  name: string
): Observable<T> {
  data.subscribe({
    next: (response: T) => {
      localStorage.setItem(name, JSON.stringify(response));
      localStorage.setItem(name + '-timestamp', `${Date.now()}`);
    },
  });
  return data;
}

export function readFromCache<T>(
  name: string,
  callback: () => Observable<T>
): Observable<T> {
  let data: Observable<T>;
  if (isDataOutdated(name)) {
    data = storeInCache<T>(callback(), name);
  } else {
    let dataFromCache = localStorage.getItem(name);
    try {
      data = of(JSON.parse(dataFromCache));
    } catch (err) {
      console.error(
        `Could not parse ${JSON.stringify(
          dataFromCache
        )}: ${err}. Fetching again data from callback function.`
      );
      data = storeInCache<T>(callback(), name);
    }
  }
  return data;
}
