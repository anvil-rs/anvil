import React from 'react';

interface UserProfileProps {

  username: string;

  avatar: string;

  bio: string;

}

export const UserProfile = (props: UserProfileProps) => {
  return (
    <div>
      <h2>UserProfile</h2>
      <ul>
      
        <li>username: string</li>
      
        <li>avatar: string</li>
      
        <li>bio: string</li>
      
      </ul>
    </div>
  );
}

export default UserProfile;