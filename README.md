# **League of Legends Esports Team Scouting Tool**  

**Overview**  
- A tool designed for competitive League of Legends teams to scout opponents by generating summaries of their champion pools.  
- Built with Rust and PostgreSQL, integrating Riot API for data retrieval.  
- Includes a simple HTML interface for parsing OPGG Multi links and fetching player IDs (PUUIDs).  

**Features**  
- Parse an OPGG Multi link into team information.  
- Fetch and display player PUUIDs for further data retrieval.  
- [Future Feature] Analyze match histories to generate a champion pool for each player based on defined metrics.  

**Technologies Used**  
- **Backend**: Rust  
- **Database**: PostgreSQL  
- **APIs**: Riot API  
- **Frontend**: HTML 

**Setup Instructions**  
1. Clone this repository: `git clone [repo_url]`.  
2. Install dependencies: `[commands for dependencies, if any]`.  
3. Run the backend server: `[commands]`.  
4. Access the HTML form at `http://localhost:[port]`.  

**Next Steps**  
- Define metrics for champion pool analysis.  
- Implement champion pool calculation logic.  
- Expand the front-end interface for user-friendly results using React.js.  
