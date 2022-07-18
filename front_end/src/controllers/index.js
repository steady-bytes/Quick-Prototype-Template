import { request } from 'graphql-request'
import useSWR from 'swr'

const fetcher = query => request("/v1/graphql", query)

// useGraphQuery - is a wrapper around `SWR` so that we can change cache layers in the future if needed
export function useGraphQuery (query) {
  const { data, error } = useSWR(query, fetcher)

  return {
    data: data,
    isLoading: !error && !data,
    isError: error
  }
}

// usePages - gathers all of the pages a user is able to see if a problem occures then and `error` will be returned
export function usePages() {
  const {data, error} = useGraphQuery(`
    {
      front_end_metadata_pages {
        id
        name
        path
        version
        created_at
        updated_at
      }
    }`)

  return {
    data: data,
    isLoading: !error && !data,
    isError: error
  }
}
