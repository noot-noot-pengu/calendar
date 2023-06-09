import { configureStore } from '@reduxjs/toolkit'
import thisMonthReducer from './contexts/thisMonth'
import userDataReducer from './contexts/userData'
import eventsDataReducer from './contexts/eventsData'




export const store = configureStore({
  	reducer: {
  		calendar: thisMonthReducer,
  		user_data: userDataReducer,
  		events_data: eventsDataReducer,
  	},
})

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch