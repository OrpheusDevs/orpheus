// components/ArtistSection.js
import React from 'react';
import { motion } from 'framer-motion';

const ArtistSection = () => {
  const artists = [
    {
      name: "ElectroNebula",
      genre: "Electronic",
      avatar: "/api/placeholder/100/100",
      followers: "12.5K",
      nfts: 7
    },
    {
      name: "Luna Symphony",
      genre: "Orchestral",
      avatar: "/api/placeholder/100/100",
      followers: "8.2K",
      nfts: 5
    },
    {
      name: "Quantum Beats",
      genre: "EDM",
      avatar: "/api/placeholder/100/100",
      followers: "21.3K",
      nfts: 12
    },
    {
      name: "Cosmic Rhythms",
      genre: "Ambient",
      avatar: "/api/placeholder/100/100",
      followers: "15.7K",
      nfts: 9
    }
  ];

  return (
    <section className="py-20 bg-[#0a0a2a]">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
          viewport={{ once: true }}
          className="text-center mb-16"
        >
          <h2 className="text-4xl font-bold mb-4">Top Artists on Orpheus</h2>
          <p className="text-xl text-purple-300 max-w-3xl mx-auto">
            Discover talented artists creating exclusive NFT music collections
          </p>
        </motion.div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {artists.map((artist, index) => (
            <motion.div
              key={artist.name}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              viewport={{ once: true }}
              className="bg-gradient-to-b from-[#1a1a4a] to-[#0a0a2a] rounded-xl p-6 shadow-xl"
            >
              <div className="flex flex-col items-center">
                <div className="relative w-24 h-24 mb-4">
                  <img 
                    src={artist.avatar} 
                    alt={artist.name}
                    className="rounded-full object-cover"
                  />
                  <div className="absolute bottom-0 right-0 bg-purple-600 rounded-full w-6 h-6 flex items-center justify-center border-2 border-[#0a0a2a]">
                    <svg className="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                    </svg>
                  </div>
                </div>
                <h3 className="text-xl font-bold mb-1">{artist.name}</h3>
                <p className="text-purple-400 text-sm mb-4">{artist.genre}</p>
                
                <div className="flex justify-between w-full mb-4">
                  <div className="text-center">
                    <p className="text-sm text-gray-400">Followers</p>
                    <p className="font-bold">{artist.followers}</p>
                  </div>
                  <div className="text-center">
                    <p className="text-sm text-gray-400">NFTs</p>
                    <p className="font-bold">{artist.nfts}</p>
                  </div>
                </div>
                
                <button className="w-full bg-transparent hover:bg-purple-900 hover:bg-opacity-30 text-white py-2 px-4 border border-purple-500 rounded transition duration-300">
                  View Collection
                </button>
              </div>
            </motion.div>
          ))}
        </div>
        
        <motion.div
          className="mt-12 text-center"
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          transition={{ duration: 0.8, delay: 0.4 }}
          viewport={{ once: true }}
        >
          <button className="bg-transparent hover:bg-purple-900 hover:bg-opacity-30 text-white font-bold py-3 px-8 border-2 border-purple-500 rounded-full transition duration-300">
            Discover More Artists
          </button>
        </motion.div>
      </div>
    </section>
  );
};

export default ArtistSection;