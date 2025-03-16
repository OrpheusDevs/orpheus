// components/NFTGallery.js
"use client"
import React, { useState } from 'react';
import { motion } from 'framer-motion';

const NFTGallery = () => {
  const [hoveredNft, setHoveredNft] = useState(null);
  
  const nfts = [
    {
      id: 1,
      title: "Cosmic Vibrations",
      artist: "ElectroNebula",
      image: "/api/placeholder/300/300",
      price: "2.4 SOL",
      edition: "Limited Edition",
      timeLeft: "8h 45m"
    },
    {
      id: 2,
      title: "Lunar Harmony",
      artist: "Luna Symphony",
      image: "/api/placeholder/300/300",
      price: "1.8 SOL",
      edition: "1 of 10",
      timeLeft: "2d 12h"
    },
    {
      id: 3,
      title: "Digital Pulse",
      artist: "Quantum Beats",
      image: "/api/placeholder/300/300",
      price: "3.5 SOL",
      edition: "1 of 5",
      timeLeft: "4h 20m"
    },
    {
      id: 4,
      title: "Ethereal Echoes",
      artist: "Cosmic Rhythms",
      image: "/api/placeholder/300/300",
      price: "1.2 SOL",
      edition: "1 of 15",
      timeLeft: "1d 6h"
    }
  ];

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
      {nfts.map((nft, index) => (
        <motion.div
          key={nft.id}
          className="relative group"
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: index * 0.1 }}
          viewport={{ once: true }}
          onMouseEnter={() => setHoveredNft(nft.id)}
          onMouseLeave={() => setHoveredNft(null)}
        >
          <div className="bg-[#1a1a4a] rounded-xl overflow-hidden shadow-lg transition-all duration-300 transform group-hover:scale-[1.02] border border-purple-800 group-hover:border-purple-600 group-hover:shadow-purple-900/20 group-hover:shadow-xl">
            <div className="relative">
              <img 
                src={nft.image} 
                alt={nft.title}
                className="w-full h-64 object-cover"
              />
              <div className="absolute top-3 right-3 bg-purple-600 text-white text-xs font-bold px-2 py-1 rounded-full">
                {nft.edition}
              </div>
              
              {/* Animated waveform overlay */}
              <div className={`absolute inset-0 bg-gradient-to-t from-[#0a0a2a] to-transparent opacity-30 flex items-end justify-center pb-4 transition-opacity duration-300 ${hoveredNft === nft.id ? 'opacity-80' : 'opacity-30'}`}>
                <div className="flex items-end space-x-1 h-12">
                  {[...Array(12)].map((_, i) => (
                    <motion.div
                      key={i}
                      className="w-1 bg-purple-400"
                      initial={{ height: 4 }}
                      animate={{ 
                        height: hoveredNft === nft.id 
                          ? Math.random() * 24 + 4 
                          : 4 
                      }}
                      transition={{
                        duration: 0.2,
                        repeat: hoveredNft === nft.id ? Infinity : 0,
                        repeatType: "reverse"
                      }}
                    />
                  ))}
                </div>
              </div>
            </div>
            
            <div className="p-5">
              <div className="flex justify-between items-start mb-2">
                <div>
                  <h3 className="text-lg font-bold text-white truncate">{nft.title}</h3>
                  <p className="text-purple-400 text-sm">by {nft.artist}</p>
                </div>
                <div className="bg-[#0a0a2a] rounded-lg px-2 py-1">
                  <p className="text-green-400 font-bold">{nft.price}</p>
                </div>
              </div>
              
              <div className="mt-4 flex justify-between items-center">
                <div>
                  <p className="text-xs text-gray-400">Auction ends in</p>
                  <p className="text-white font-mono">{nft.timeLeft}</p>
                </div>
                
                <motion.button 
                  className="bg-purple-600 hover:bg-purple-700 text-white py-2 px-4 rounded-lg text-sm font-medium"
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  Place Bid
                </motion.button>
              </div>
            </div>
            
            {/* Play button overlay */}
            <motion.div 
              className={`absolute inset-0 flex items-center justify-center transition-opacity duration-300 ${hoveredNft === nft.id ? 'opacity-100' : 'opacity-0'} pointer-events-none`}
              initial={{ opacity: 0 }}
              animate={{ opacity: hoveredNft === nft.id ? 1 : 0 }}
            >
              <motion.div 
                className="bg-white bg-opacity-20 backdrop-blur-sm rounded-full p-3"
                whileHover={{ scale: 1.1 }}
              >
                <svg className="w-10 h-10 text-white" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" />
                </svg>
              </motion.div>
            </motion.div>
          </div>
        </motion.div>
      ))}
    </div>
  );
};

export default NFTGallery;