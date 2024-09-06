
export const statusCodeMessages: { [key: number]: string } = {
    400: 'Bad request. Please check your input and try again.',
    401: 'Invalid credentials.',
    403: 'Forbidden. You do not have permission to perform this action.',
    404: 'Resource not found. Please check the URL or try again later.',
    500: 'An error occurred on the server. Please try again later.',
    502: 'Bad gateway. The server received an invalid response from an upstream server.',
    503: 'Service is temporarily unavailable. Please try again later.',
    504: 'The server took too long to respond. Please try again later.',
  };
  
  