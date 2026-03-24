const initialState = {
  isCollapse: false,
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const utilsReducer = (state = initialState, action: any) => {
  switch (action.type) {
    case "SET_COLLAPSE": {
      const payload: boolean = action.payload
      return {
        ...state,
        isCollapse: payload,
      }
    }
    default:
      return state
  }
}
