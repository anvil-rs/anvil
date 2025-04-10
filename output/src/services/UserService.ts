/**
 * UserService - TypeScript Service
 * 
 * Auto-generated service class
 */

export class UserService {
  
  async getUser() {
    // Implementation for getUser
    return await fetch(`/api/${this.resourceName}`);
  }
  
  async updateUser() {
    // Implementation for updateUser
    return await fetch(`/api/${this.resourceName}`);
  }
  
  async deleteUser() {
    // Implementation for deleteUser
    return await fetch(`/api/${this.resourceName}`);
  }
  

  private get resourceName() {
    return 'userservice';
  }
}
