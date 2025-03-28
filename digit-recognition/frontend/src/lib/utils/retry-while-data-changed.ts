import { QueryObserverResult } from "@tanstack/react-query";

type Params = {
  data: unknown;
  refetch: () => Promise<QueryObserverResult<unknown | undefined, Error>>;
  maxRetries?: number;
};

const retryWhileDataChanged = ({ data, refetch, maxRetries = 15 }: Params) =>
  new Promise<void>((resolve) => {
    const prevData = JSON.stringify(data);

    const retry = async (attempt = 0) => {
      if (attempt >= maxRetries) {
        console.log("maxRetries exceeded", maxRetries);
        resolve();
        return;
      }

      const response = await refetch();
      const isSameData = JSON.stringify(response.data) === prevData;

      if (isSameData) {
        setTimeout(() => retry(attempt + 1), 1000);
      } else {
        console.log("resolved on attempt", attempt);
        resolve();
      }
    };

    retry();
  });

export { retryWhileDataChanged };
