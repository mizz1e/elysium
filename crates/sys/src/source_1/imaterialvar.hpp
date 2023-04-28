#pragma once
#include "math.hpp"

// opaque types
struct ITexture;
struct VMatrix;

// defined types
struct IMaterial;
struct IMaterialVar;

// TODO: CUtlSymbol.
struct CUtlSymbol {
    char *symbol;
};

enum MaterialVarType_t { 
    MATERIAL_VAR_TYPE_FLOAT = 0,
    MATERIAL_VAR_TYPE_STRING,
    MATERIAL_VAR_TYPE_VECTOR,
    MATERIAL_VAR_TYPE_TEXTURE,
    MATERIAL_VAR_TYPE_INT,
    MATERIAL_VAR_TYPE_FOURCC,
    MATERIAL_VAR_TYPE_UNDEFINED,
    MATERIAL_VAR_TYPE_MATRIX,
    MATERIAL_VAR_TYPE_MATERIAL,
};

typedef unsigned short MaterialVarSym_t;

struct IMaterialVar {
    typedef unsigned long FourCC;

    char *m_pStringVal;
    int m_intVal;
    Vec4 m_VecVal;
    uint8_t m_Type : 4;
    uint8_t m_nNumVectorComps : 3;
    uint8_t m_bFakeMaterialVar : 1;
    uint8_t m_nTempIndex;
    CUtlSymbol m_Name;

    virtual ITexture *GetTextureValue() = 0;
    virtual bool IsTextureValueInternalEnvCubemap() const = 0;
    virtual char const *GetName() const = 0;
    virtual MaterialVarSym_t GetNameAsSymbol() const = 0;
    virtual void SetFloatValue(float val) = 0;
    virtual void SetIntValue(int val) = 0;
    virtual void SetStringValue(char const *val) = 0;
    virtual char const *GetStringValue() const = 0;
    virtual void SetFourCCValue(FourCC type, void *pData) = 0;
    virtual void GetFourCCValue(FourCC *type, void **ppData) = 0;
    virtual void SetVecValue(float const *val, int numcomps) = 0;
    virtual void SetVecValue(float x, float y) = 0;
    virtual void SetVecValue(float x, float y, float z) = 0;
    virtual void SetVecValue(float x, float y, float z, float w) = 0;
    virtual void GetLinearVecValue(float *val, int numcomps) const = 0;
    virtual void SetTextureValue(ITexture *) = 0;
    virtual IMaterial *GetMaterialValue() = 0;
    virtual void SetMaterialValue(IMaterial *) = 0;
    virtual bool IsDefined() const = 0;
    virtual void SetUndefined() = 0;
    virtual void SetMatrixValue(VMatrix const& matrix) = 0;
    virtual const VMatrix &GetMatrixValue() = 0;
    virtual bool MatrixIsIdentity() const = 0;
    virtual void CopyFrom(IMaterialVar *pMaterialVar) = 0;
    virtual void SetValueAutodetectType(char const *val) = 0;
    virtual IMaterial *GetOwningMaterial() = 0;
    virtual void SetVecComponentValue(float fVal, int nComponent) = 0;
    virtual int GetIntValueInternal() const = 0;
    virtual float GetFloatValueInternal() const = 0;
    virtual float const *GetVecValueInternal() const = 0;
    virtual void GetVecValueInternal(float *val, int numcomps) const = 0;
    virtual int VectorSizeInternal() const = 0;
};
